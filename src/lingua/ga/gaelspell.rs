use libperl_rs::*;
use libperl_sys::*;
use std::cell::RefCell;
use std::convert::TryInto;
use std::ffi::CString;

thread_local! {
    static PERL: RefCell<Option<Perl>> = RefCell::new(None);
}

fn ensure_perl_inited() -> Result<(), String> {
    PERL.with(|cell| {
        if cell.borrow().is_some() {
            return Ok(());
        }

        let mut perl = Perl::new();

        // Load our bridge module that adapts GaelSpell API.
        let r1 = perl.parse(&["", "-MGaelSpellBridge", "-e0"], &[]);
        if r1 != 0 {
            return Err("Failed to load GaelSpellBridge (perl.parse returned non-zero)".into());
        }

        // Verify method exists
        let can = call_list_method(&mut perl, "GaelSpellBridge".into(), "can".into(), vec!["spellcheck_json".to_string()])?;
        if can.is_empty() {
            return Err("GaelSpellBridge->spellcheck_json not found (method missing)".into());
        }

        *cell.borrow_mut() = Some(perl);
        Ok(())
    })
}

/// Health check call: ask bridge to check a single word (or init only)
pub fn check_word(_word: &str) -> Result<(), String> {
    if std::env::var("GAELSPELL_DISABLE_PERL").ok().as_deref() == Some("1") {
        return Ok(());
    }
    ensure_perl_inited()
}

/// Spellcheck text; return Vec of [misspelling, suggestions[]] pairs
pub fn spellcheck(text: &str) -> Result<Vec<(String, Vec<String>)>, String> {
    if std::env::var("GAELSPELL_DISABLE_PERL").ok().as_deref() == Some("1") {
        return Ok(vec![]);
    }
    ensure_perl_inited()?;
    PERL.with(|cell| {
        let mut binding = cell.borrow_mut();
        let perl: &mut Perl = binding.as_mut().expect("Perl not initialised");
        let json = call_scalar_method(perl, "GaelSpellBridge".into(), "spellcheck_json".into(), vec![text.to_owned()])?;
        let s = json.unwrap_or_else(|| "[]".to_string());
        let v: serde_json::Value = serde_json::from_str(&s).map_err(|e| e.to_string())?;
        let arr = v.as_array().ok_or_else(|| "expected JSON array".to_string())?;
        let mut out = Vec::with_capacity(arr.len());
        for item in arr {
            let pair = item.as_array().ok_or_else(|| "expected pair array".to_string())?;
            if pair.len() != 2 { return Err("pair wrong length".to_string()); }
            let w = pair[0].as_str().ok_or_else(|| "word not string".to_string())?.to_string();
            let sugs_v = pair[1].as_array().ok_or_else(|| "sugs not array".to_string())?;
            let sugs: Vec<String> = sugs_v.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect();
            out.push((w, sugs));
        }
        Ok(out)
    })
}

fn sv_extract_pv(sv: *const libperl_sys::sv) -> Option<String> {
    let ptr = unsafe { (*sv).sv_u.svu_pv };
    if !ptr.is_null() { Some (unsafe {std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()}) } else { None }
}

fn call_scalar_method(perl: &mut Perl, class: String, method: String, args: Vec<String>) -> Result<Option<String>, String> {
    let my = perl.my_perl();

    unsafe_perl_api! { Perl_push_scope(my) }
    unsafe_perl_api! { Perl_savetmps(my) }

    let mut sp = unsafe { (*my).Istack_sp };
    perl.pushmark(sp);
    sp = unsafe_perl_api! { Perl_stack_grow(my, sp, sp, (1 + args.len()).try_into().unwrap()) };
    sp_push!(sp, perl.str2svpv_mortal(class.as_str()));
    for a in &args { sp_push!(sp, perl.str2svpv_mortal(a)); }
    unsafe { (*my).Istack_sp = sp; }

    let c_method = CString::new(method).map_err(|_| "null in method".to_string())?;
    let count = unsafe_perl_api! { Perl_call_method(my, c_method.as_ptr(), (G_METHOD_NAMED|G_SCALAR) as i32) };
    let sp_after = unsafe { (*my).Istack_sp };

    let res = if count == 0 { None } else {
        let sv = unsafe { *sp_after };
        sv_extract_pv(sv)
    };

    unsafe { (*my).Istack_sp = sp_after.sub(count as usize); }
    perl.free_tmps();
    unsafe_perl_api! { Perl_pop_scope(my) }
    Ok(res)
}

fn call_list_method(perl: &mut Perl, class: String, method: String, args: Vec<String>) -> Result<Vec<*const libperl_sys::sv>, String> {
    let my = perl.my_perl();

    unsafe_perl_api! { Perl_push_scope(my) }
    unsafe_perl_api! { Perl_savetmps(my) }

    // dSP
    let mut sp = unsafe { (*my).Istack_sp };

    // PUSHMARK(SP)
    perl.pushmark(sp);

    // XPUSHs(invocant + args)
    sp = unsafe_perl_api! { Perl_stack_grow(my, sp, sp, (1 + args.len()).try_into().unwrap()) };
    sp_push!(sp, perl.str2svpv_mortal(class.as_str()));
    for a in &args { sp_push!(sp, perl.str2svpv_mortal(a)); }

    // PUTBACK
    unsafe { (*my).Istack_sp = sp; }

    // call
    let c_method = CString::new(method).map_err(|_| "null in method".to_string())?;
    let count = unsafe_perl_api! { Perl_call_method(my, c_method.as_ptr(), (G_METHOD_NAMED|G_LIST) as i32) };

    // SPAGAIN
    let sp_after = unsafe { (*my).Istack_sp };

    // collect
    let mut res: Vec<*const libperl_sys::sv> = Vec::with_capacity(count as usize);
    let mut src = unsafe { sp_after.sub(count as usize - 1) } as *const *mut libperl_sys::sv;
    for _ in 0..count {
        let sv = unsafe { *src } as *const libperl_sys::sv;
        res.push(sv);
        src = unsafe { src.add(1) };
    }

    // pop
    unsafe { (*my).Istack_sp = sp_after.sub(count as usize); }

    perl.free_tmps();
    unsafe_perl_api! { Perl_pop_scope(my) }
    Ok(res)
}

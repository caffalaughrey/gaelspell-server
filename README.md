# gaelspell-server

Lean server hosting GaelSpell (https://cadhan.com/gaelspell/)

## Build
```bash
make
```

## Run
```bash
docker run --rm -p 5051:5000 caffalaughrey/gaelspell
```

## Use
```bash
curl -X POST localhost:5051/api/gaelspell/1.0 \
  -H "Content-Type: application/json" \
  --data '{"teacs": "Ba mhath liom abcdefxyz"}'
```

Notes
- One worker process (Perl embedded), like `gramadoir-server`.
- UTF-8 JSON responses; suggestions come from Hunspell built with GaelSpell data.

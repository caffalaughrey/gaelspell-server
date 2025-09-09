use ExtUtils::MakeMaker;
WriteMakefile(
    NAME         => 'GaelSpellBridge',
    VERSION_FROM => 'lib/GaelSpellBridge.pm',
    PREREQ_PM    => { 'Text::Hunspell' => 0, 'JSON::PP' => 0 },
    ($] >= 5.005 ?    ## Add these new keywords supported since 5.005
      (ABSTRACT_FROM => 'lib/GaelSpellBridge.pm', # retrieve abstract from module
       AUTHOR     => 'Auto-generated') : ()),
);



package GaelSpellBridge;
use strict;
use warnings;

use Text::Hunspell ();    # uses GaelSpell hunspell .aff/.dic
use JSON::PP ();
use Encode qw(decode);

my $SPELLER;

sub _get {
    return $SPELLER if $SPELLER;
    my $aff = $ENV{GAELSPELL_AFF} // '/usr/share/hunspell/ga_IE.aff';
    my $dic = $ENV{GAELSPELL_DIC} // '/usr/share/hunspell/ga_IE.dic';
    my $s = Text::Hunspell->new($aff, $dic);
    return undef unless $s;
    $SPELLER = $s;
}

# Returns JSON string of [word, [sug...]] pairs
sub spellcheck_json {
    my ($class, $text) = @_;
    my $pairs = eval {
        my $s = _get() or die "hunspell init failed";
        # Determine encoding from aff SET if available
        my $aff = $ENV{GAELSPELL_AFF} // '/usr/share/hunspell/ga_IE.aff';
        my $enc = 'UTF-8';
        if (open my $fh, '<', $aff) {
            while (my $l = <$fh>) {
                if ($l =~ /^\s*SET\s+(\S+)/i) { $enc = $1; last; }
            }
            close $fh;
        }
        my @pairs;
        while ($text =~ /([\p{L}ÁÉÍÓÚáéíóú'-]+)/g) {
            my $w = $1;
            next if $s->check($w);
            my @sugs = $s->suggest($w);
            @sugs = map { decode($enc, $_) } @sugs;
            push @pairs, [$w, \@sugs];
        }
        \@pairs;
    };
    if ($@) {
        return JSON::PP::encode_json([]);
    }
    return JSON::PP::encode_json($pairs);
}

1;



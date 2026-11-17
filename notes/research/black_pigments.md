# Vine black against bone black: how strong a black prints

Tags: [M] measured, [P] practitioners and makers, [D] derived, [E] estimate.

## The question

An etching ink of vine black printed much weaker than one of bone black. The tube data come from
upstream main: bone black's masstone in oil is the Revigo measurement (near zero reflectance),
hiding 0.9; vine black's was an estimate, #323538 (Y 3.5 %, L* 22), hiding 0.75, tinting strength
0.6 ("charcoal black: bluish, weak"). An ink's absorption per coat is its tube's,
K = S·(1 − R∞)²/(2R∞), S from the masstone and hiding (`pigment::scatter_for`). With main's data a
coat of vine black's tube paste absorbed 0.49 of a coat of bone black's (the impressionist box),
and per volume of pigment 0.25 (vine black's tube paste holds 0.45 pigment, bone black's 0.22).

## What is measured

- **Masstone in oil.** No measured R∞ of a true vine (PBk8) black in oil was found. Cosentino's
  FORS database of historical pigments in linseed oil (e-conservation Journal 2, 2014; spectra from
  CHSOS) reads ivory 15.7 %, bone 16.4 %, vine 16.7 % and lamp black 18.4 % in Y: the set sits on an
  instrument floor (its titanium white reads 113 %), but the blacks are indistinguishable on it, so
  vine black's masstone is within about half a per cent of bone black's [M, with that caveat].
  Michael Harding's "Vine Black" oil measures L* 8.4 (artistpigments.org), though its pigment is
  listed as PBk11 [M]. CHSOS's acrylic swatch labelled vine black (Kremer 47000) is anthracite by its
  maker's SDS and does not cover its card; it says nothing about strength.
- **Strength.** No absolute K and S of carbon or bone blacks in oil was found. Period and makers'
  words: blue (vine) black "of weaker body than ivory or lamp black", Frankfort black (vine lees)
  "next in intensity to lamp black, and more powerful than that of ivory" (Field's *Chromatography*,
  Salter's 1869 edition, §290, §300) [P]; charcoal black's tinting strength lower than lamp or ivory
  black (Essential Vermeer; Jackson's 2021) [P]; Lumsden (*The Art of Etching*, 1925, pp. 85–86):
  buy Frankfort black "of a good colour—some is very grey"; French black is "very rich, dense", the
  Frankfort blacks "less dense" and they wipe off the plate more freely [P]. Kern et al. (J. Paper
  Conservation 21(2), 2020) found a reconstructed lees-black ink "deep dark" and like modern inks in
  print [M, qualitative].
- **Particles.** Bone black is about 10 % carbon on 84 % hydroxyapatite (Gettens & Stout 1966,
  via CAMEO) [M], its particles 3–15 µm; vine black is opaque splinters showing the plant's cells,
  some 5–20 µm [P]; charcoal's skeletal density 1.34–1.96 g/cm³ (Brewer et al. 2014) [M].

## The physics

Carbon's index in the visible is about 1.95 − 0.79i (Bond & Bergstrom 2006) [M], so light goes
about λ/(4πk) ≈ 0.055 µm into it, and a grain of either black, whole carbon or carbon over apatite,
is opaque. An opaque grain absorbs about its cross-section: per volume of pigment ~1.5/d, whatever
its carbon content [D]. So at equal pigment volume vine black absorbs as bone black's size over its
own, about 0.6 (0.3–1.0) [E from the sizes]; at equal mass, its lower density brings it near
parity, and vine black's tube carries more pigment to the volume (its oil absorption is low). So
vine black should print somewhat weaker and greyer than bone black, as the printers say, and not
much weaker.

## What the engine does now (`--@ vine-black`)

Main's data put vine black's absorption per volume of pigment at 0.25 of bone black's, below the
range, through a masstone (L* 22) no measurement supports. Under the switch (every new painting;
`Palette::vine_black`) vine black's masstone in oil is #202224 (Y 1.6 %, L* 13, carbon's slight blue
kept) [E: within the Cosentino floor of bone black's, between Harding's L* 8.4 and a grey Frankfort
black]. Its hiding (0.75, semi-opaque) and tinting strength (0.6) stand. A coat of its tube's paste
then absorbs 0.60 of a coat of bone black's, as its own tinting strength says, and 0.30 per volume
of pigment, the low end of the size argument: weaker, as the period sources say, but not the 0.49 it
was. A copperplate ink of it (`paste="copperplate"`) holds 0.52 pigment against bone black's 0.34
(its critical share is higher), so the two copperplate blacks come closer.

A log without the line replays vine black as it was painted; the save keeps the switch. The
engine word needs no bit: the switch is the palette's, as `--@ dry-lakes` is (the 60th bit,
reserved for this fix, stays free).

## Sources

- M. Cosentino, "FORS spectral database of historical pigments in different binders",
  *e-conservation Journal* 2, 57–68 (2014); spectra via Cultural Heritage Science Open Source
  (chsopensource.org).
- G. Field, *Chromatography*, ed. T. W. Salter (1869), §290, §300 (Project Gutenberg 20915).
- D. Lumsden, *The Art of Etching* (1925), pp. 85–86.
- Kern, Stijnman, Müller, Wiegand & Brückle, *Journal of Paper Conservation* 21(2) (2020).
- R. J. Gettens, G. L. Stout, *Painting Materials* (1966), via MFA CAMEO "Bone black".
- T. C. Bond, R. W. Bergstrom, "Light absorption by carbonaceous particles", *Aerosol Science and
  Technology* 40, 27 (2006).
- C. E. Brewer et al., *Biomass and Bioenergy* (2014): charcoal skeletal density.
- Michael Harding, Vine Black oil colour (measured L*a*b* via artistpigments.org).

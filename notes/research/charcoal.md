# Charcoal materials

Research read October 18, 2026. These are relative drawing-material presets,
not measured replicas of a manufacturer's product. The stick, tooth, wear,
crumb transport, finger, eraser and fixative remain the dry-media stack's models.

## Sources

- [Utrecht, The Utrecht Guide to Charcoal](https://www.utrechtart.com/info/artist-resources/guide-to-charcoal/):
  vine and willow are carbonized branches with **no added binder**. They
  erase readily and need fixative. Compressed charcoal adds grease, wax or
  starch binders, produces darker marks and is much harder to erase.
  Charcoal pencils have compressed-charcoal cores, not graphite cores.
- [Jackson's, The Difference Between Graphite and Charcoal Explained](https://www.jacksonsart.com/blog/2018/08/23/difference-between-graphite-and-charcoal/):
  vine is dark gray, willow darker and finer-grained. Raw charcoal fractures
  irregularly and sheds dust. Compressed charcoal uses powder and gum or wax,
  is less dusty and harder to erase. More binder makes a harder grade.
  Powder tones areas with a brush and is generally lighter than compressed.
- [Coates, products](https://www.coatescharcoal.co.uk/products.htm):
  natural willow sticks vary in diameter; thin sticks are approximately
  2–3 mm. [Its size table](https://www.coatescharcoal.co.uk/artists.htm)
  also lists thin 3–4 mm and thick 7–9 mm. Presets use 3 and 8 mm.
- [Nitram, powdered charcoal](https://nitramcharcoal.com/products/nitram-powdered-charcoal):
  states a uniform **100 µm particle size** and brush or paper-stump application.
  This is a commercial powder/agglomerate size, not evidence that every
  charcoal fragment or flattened deposit is 100 µm thick.
- [Strathmore, 300 Newsprint](https://www.strathmoreartist.com/draw-newsprint/300-series-newsprint):
  natural-toned 52 g/m² paper, rough finish suitable for charcoal.
- [Strathmore, 500 Charcoal](https://www.strathmoreartist.com/draw-charcoal/500-series-charcoal):
  95 g/m², 100% cotton, laid finish, available in tints including smoke gray.
- [Strathmore, 300 Bristol](https://www.strathmoreartist.com/draw-bristol/300-series-bristol):
  270 g/m²; smooth has little to no tooth, unlike vellum. Charcoal therefore
  has less texture to grip and reaches its layering limit sooner.

## Numerical interpretation

No source above supplies artists' charcoal yield hardness, Archard wear,
reflectance spectra, binder mass fraction or eraser removal coefficients.
The following numbers are **model estimates**, chosen to preserve the sourced
ordering. Binder is an effective adhered share, not a recipe's weight percent.
Reflectance is linear RGB and therefore is not an sRGB gray swatch value.

| Material | Reflectance | Hardness MPa | Wear | Shed share | Deposit µm | Lift | Adhered share |
|---|---:|---:|---:|---:|---:|---:|---:|
| Vine, thin/thick | .055 | 1.2 | .09 | .94 | 9 | .95 | 0 |
| Willow, thin/thick | .032 | 1.5 | .08 | .92 | 6 | .92 | 0 |
| Compressed | .006 | 5 | .045 | .55 | 4 | .32 | .30 |
| Pencil soft | .008 | 4 | .05 | .65 | 4 | .40 | .20 |
| Pencil medium | .014 | 9 | .032 | .48 | 4 | .30 | .35 |
| Pencil hard | .025 | 18 | .018 | .32 | 4 | .22 | .50 |
| Powder | .040 | — | — | loose | 6 | .95 | 0 |

Thin and thick change geometry, not chemistry. Pencils use the existing
sharpened core. Natural charcoal's zero adhered share leaves all volume
available to the finger and eraser. Compressed cores leave an adhered fraction
in the tooth and shed less. Fixative still binds the remaining loose dust.
The material's effective lift coefficient travels with its deposited color.
The 6–9 µm deposit thickness represents crushed particles spread by a tool;
powder's crumb upper size is 100 µm, following Nitram's stated powder size.

Paper grammage and finish are sourced above. Fiber dimensions, pore depth,
calendering, felt depth, stiffness, flocculation and the neutral paper colors
are estimates within #79's sheet model. Newsprint is thin, irregular and warm;
toned laid paper exposes a pronounced laid tooth; smooth bristol is heavily
calendered with shallow pores. These are starting papers, not brand simulations.

White heightening reuses `pastel(pile{{"lead white",1}})`: it already deposits
light-colored dry particles on toned paper. No new white pigment or graphite
chalk behavior is needed. `chalk()` remains the existing black chalk.

Compatibility: existing pastel constructors retain exactly their original
mechanics, particle thickness, shed share and lift. Charcoal globals, their
material properties and paper presets require opt-in `--@ engine 9` plus
`--@ paper`. The existing exact global-exposure test for the paper switch
would fail if new globals were added to engines 5, 7 or 8. Engine 9 preserves
engine 8's paint behavior while allowing these additions; the default stays
engine 8. No checkpoint layout or exact baseline changes.

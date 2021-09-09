# Workspace orbtk_tinyskia

[<img src="img/orbskia.svg" width="720"/>](img/orbskia.svg)

Rendering is a key component of the toolkit. Everybody is expecting state
of the art presentation of implemented widgets. User interaction that will
result in layout and entity changes inside the GUI should be updated as
soon as possible. A comfortable user experience is mainly influenced by
fast rendering tasks. New rendering of layouts should only take place, if
constraint changes will need to do so. Entities and their attributes will
only require new rendering if a user interaction changes their state
to be dirty.

Lets summarize the main goals of `OrbTK` rendering infrastructure:

* API encapsulated access to all renderer functions

  This design decision is taken to keep freedom for further development of
  OrbTK when it comes to support different renderers. We are able to support

	* different versions of a given renderer
	* support different renderer for different target platforms

* 2D rendering

  We need a fast and complete implementation of all rendering
  functions that are supported in the OrbTK toolkit. The following
  summary is a list of `tiny-skia` provided functions:

  * Pixmaps
  * Canvas
  * Path
  * geometry primitives
  * Blending modes
  * Path filling
  * Anti-aliased Path filling
  * Path stroking
  * Path hairline stroking
  * Anti-aliased Path hairline stroking
  * Stroke dashing
  * Gradients (linear and radial)
  * Pixmaps blending (image on image rendering)
  * Patterns
  * Fill rect
  * Stroke rect
  * Rectangular clipping
  * Clipping
  * Anti-aliased clipping
  * Analytical anti-aliased Path filling
  * Dithering
  * Blending modes

We are looking forward to a Rust native ecosystem that handles `text
rendering`. This is a complex task and by the time of writing a
complete library addressing this issue isn't available.
The Rust community has developed building blocks, like

* [X] a Font parser: ttf-parser.
* [X] a Text shaper: rustybuzz or all-sorts.
* [X] a Font database: fontdb (supporting a font fallback mechanism).

The missing peace, beside the glue code to use the components inside
orbtk_tinyskia is a high-quality `glyph rasterization
library`. Preferably it will offer a `FreeType` level of
quality. `ab_glyph_rasterizer` or `fontdue` might evolve to fill this
gap.

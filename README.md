# MapAnnot in Rust

Map Annot is a well-known tool to make drawing and measurements on maps, in particular for people into the teasure hunt called "La Chouette d'Or".

The aim of this project is to port MapAnnot in Rust as well as adding a few more functionalities.

## Actual functionalities

- Load and size backgroud
- Adding several type of geometries
- Make meausrements for distance and angles
- Add and manipulate several layers

## Future functionalities

- Capability to edit drawable values
- Load and save capabilities - WIP


## Known issues

- Tangent lines ar not always right
- Half-line direction is not predictable
- Issue for multiple layers when switching from one to another
- Issue when selecting a point on the map after a layer add been added
- Performance when loading
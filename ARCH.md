# Layered Architecture for `.mandart` JSON Handling  

## 1. Input/Output Layer (File Handling)  
This layer manages reading and writing `.mandart` JSON files.  

### Responsibilities  
- Save the `PictureDefinition` to a `.mandart` file  
- Load a `.mandart` file and convert it to `PictureDefinition`  
- Ensure file validation (handle missing or corrupted data)  

### Implementation  
- Swift: saveMandArtImageInputs(), loadMandArtFile(from:)  
- Rust: save_mandart_file(), load_mandart_file()  
- JavaScript: saveMandArtFile(), loadMandArtFile()  

## 2. Parsing Layer (JSON Conversion)  
This layer encodes and decodes JSON to/from `PictureDefinition`.  

### Responsibilities  
- Convert objects to JSON  
- Parse JSON into `PictureDefinition`  
- Ensure proper data types (e.g., Double, Int, UUID, Array<Hue>)  

### Implementation  
- Swift: JSONEncoder/Decoder for Codable structs  
- Rust: serde_json for Deserialize and Serialize  
- JavaScript: JSON.parse() and JSON.stringify()  

## 3. Data Layer (In-Memory Representation)  
This layer holds the actual objects that are used in computations.  

### Responsibilities  
- Manage `PictureDefinition` and `Hue` instances  
- Ensure correct data validation (e.g., Hue list is not empty)  
- Provide computed properties (e.g., calculatedRightNumber)  
- Trigger updates when needed (e.g., in SwiftUI via objectWillChange.send())  

### Implementation  
- Swift: @Model PictureDefinition with computed properties  
- Rust: struct PictureDefinition with impl methods  
- JavaScript: class PictureDefinition  

## 4. Computation Layer (Processing Inputs)  
This layer uses the structured data for grid calculations.  

### Responsibilities  
- Extract `ArtImageShapeInputs` from `PictureDefinition`  
- Extract `ArtImageColorInputs` from `PictureDefinition`  
- Ensure proper conversions (e.g., Double to f64, Float64Array in JS)  
- Prepare data for Mandelbrot rendering  

### Implementation  
- Swift: extractShapeInputs()  
- Rust: extract_shape_inputs()  
- JavaScript: extractShapeInputs()  

## 5. Rendering Layer (Generating Images)  
This layer converts the computed grid into an image.  

### Responsibilities  
- Run `calcGrid()` using structured inputs  
- Run `colorGrid()` to apply the hues  
- Convert output to a usable format  
  - Swift: CGImage, UIImage  
  - Rust: Vec<Vec<String>> (#RRGGBB)  
  - JavaScript: ImageData (Uint8ClampedArray)  

## Summary of Layers & Function Responsibilities  

| Layer            | Swift Functions                     | Rust Functions                  | JavaScript Functions          |  
|----------------------|--------------------------------|--------------------------------|----------------------------------|  
| 1. File Handling  | saveMandArtImageInputs()  | save_mandart_file()  | saveMandArtFile()  |  
|                      | loadMandArtFile(from:)  | load_mandart_file()  | loadMandArtFile()  |  
| 2. JSON Parsing  | toJSON(), fromJSON()  | to_json(), from_json()  | toJSON(), fromJSON()  |  
| 3. Data Handling | PictureDefinition Model | struct PictureDefinition | class PictureDefinition |  
|                      | Hue Model  | struct Hue | class Hue |  
| 4. Computation  | extractShapeInputs() | extract_shape_inputs() | extractShapeInputs() |  
| 5. Rendering   | calcGrid(), colorGrid() | calc_grid(), color_grid() | calcGrid(), colorGrid() |  


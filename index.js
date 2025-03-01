const IS_GITHUB_PAGES = window.location.hostname.includes("github.io");

// Define Base Paths
const LOCAL_BASE_PATH = "./";
const GITHUB_BASE_PATH = "https://denisecase.github.io/mandart-engine-rust/";
const BASE_PATH = IS_GITHUB_PAGES ? GITHUB_BASE_PATH : LOCAL_BASE_PATH;

// Define Input and WASM Paths
const INPUT_PATH = `${BASE_PATH}input/`;
const WASM_PATH = `${BASE_PATH}public/pkg/mandart_wasm_bg.wasm`;
const WASM_JS_PATH = `${BASE_PATH}public/pkg/mandart_wasm.js`;

// List of MandArt files to process
const MANDART_FILES = [
  "Bhj1.mandart",
  "Frame54.mandart",
  "Frame05.mandart",
  "Dragon2.mandart",
];

async function testMandartFileProcessing(wasmModule) {
  console.log("Testing Mandart file processing...");

  const fileListContainer = document.getElementById("fileListContainer");
  fileListContainer.innerHTML =
    "<strong>Processing the following files:</strong><ul></ul>";
  const fileList = fileListContainer.querySelector("ul");

  const processingResults = {
    total: MANDART_FILES.length,
    successful: 0,
    failed: 0,
    errors: [],
  };

  for (const file of MANDART_FILES) {
    try {
      const fileURL = `${INPUT_PATH}${file}`;
      console.log(`Fetching: ${fileURL}`);

      const response = await fetch(fileURL);
      if (!response.ok) {
        throw new Error(`Failed to fetch ${file}`);
      }
      const content = await response.json();

      // Add file name to the list in the UI
      const listItem = document.createElement("li");
      listItem.textContent = file;
      fileList.appendChild(listItem);

      // Use the WASM function to extract inputs
      const [shapeInputs, colorInputs] =
        wasmModule.api_get_inputs_from_picdef_string(JSON.stringify(content));

      console.log(`Processing ${file}:`, { shapeInputs, colorInputs });

      // Process file with WASM function
      const imageData = wasmModule.api_get_image_from_inputs(
        shapeInputs,
        colorInputs
      );

      // Validate image data
      if (!imageData || !imageData.data || imageData.data.length === 0) {
        throw new Error("Invalid image data generated");
      }

      // Create canvas for each processed image
      const canvas = document.createElement("canvas");
      canvas.width = imageData.width;
      canvas.height = imageData.height;
      canvas.title = file;
      canvas.style.border = "1px solid black";
      canvas.style.margin = "10px";

      const ctx = canvas.getContext("2d");
      const imageDataObj = new ImageData(
        new Uint8ClampedArray(imageData.data),
        imageData.width,
        imageData.height
      );
      ctx.putImageData(imageDataObj, 0, 0);

      document.getElementById("canvasContainer").appendChild(canvas);
      processingResults.successful++;
    } catch (error) {
      processingResults.failed++;
      processingResults.errors.push({
        file,
        error: error.message,
      });
      console.error(`Error processing ${file}:`, error);
    }
  }

  console.log("Image Processing Results:", processingResults);
  return processingResults;
}

async function init() {
  try {
    // Import the WASM module
    const wasmModule = await import(WASM_JS_PATH);
    await wasmModule.default();
    console.log("WASM module loaded successfully");

    // Run file processing
    await testMandartFileProcessing(wasmModule);
  } catch (loadError) {
    console.error("Failed to load WASM module:", loadError);
  }
}

// Run initialization when the page loads
window.addEventListener("DOMContentLoaded", init);

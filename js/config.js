/***********************************************************************************************************************
 * Porous Absorber Calculator Configuration
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

import {
  getInt
, getFloat
, getCheckbox
, getRadio
, getInnerHTML

, setInt
, setFloat
, setCheckbox
, setRadio
} from "./dom_access.js"

 /**********************************************************************************************************************
  * The tabConfig object contains one property per absorber type
  * 
  * The WASM function name exposed in lib.rs via the #[wasm_bindgen] directive determines the name of both the tabConfig
  * property listed below, and the HTML page fragment name
  * 
  * Each tabName property is an object that holds an array of the HTML elements that exist on that particular page
  * 
  * The "id" property must match the corresponding id of the input field in the DOM
  *
  * The "units" property is for metric to imperial conversion but must be maintained for all fields.
  * Use the value "each" if "units" has no particular meaning for this property
  *
  * The "getter" and "setter" properties are the function names in utils.js that get/set that particular dataype
  * 
  * The configuration tab is always present in the DOM even if the user has not explicitly selected that tab.  This
  * means that these values are always available even if those fields are not currently visible.
  *********************************************************************************************************************/
 const tabConfig = {
  "rb_porous_absorber" : [
    { id : "absorber_thickness_mm", units : "mm",      isWasmArg : true, getter : getInt,      setter : setInt      }
  , { id : "flow_resistivity",      units : "rayls/m", isWasmArg : true, getter : getInt,      setter : setInt      }
  , { id : "air_gap_mm",            units : "mm",      isWasmArg : true, getter : getInt,      setter : setInt      }
  , { id : "angle",                 units : "°",       isWasmArg : true, getter : getInt,      setter : setInt      }
  , { id : "graph_start_freq",      units : "Hz",      isWasmArg : true, getter : getFloat,    setter : setFloat    }
  , { id : "smooth_curve",          units : "each",    isWasmArg : true, getter : getCheckbox, setter : setCheckbox }
  , { id : "subdivision",           units : "each",    isWasmArg : true, getter : getRadio,    setter : setRadio    }
  ]
, "slotted_panel" : [
    { id : "panel_thickness_mm",    units : "mm",      isWasmArg : true,  getter : getFloat,     setter : setFloat    }
  , { id : "slot_distance_mm",      units : "mm",      isWasmArg : true,  getter : getFloat,     setter : setFloat    }
  , { id : "slot_width_mm",         units : "mm",      isWasmArg : true,  getter : getFloat,     setter : setFloat    }
  , { id : "slotted_porosity",      units : "each",    isWasmArg : true,  getter : getInnerHTML, setter : setFloat    }
  , { id : "absorber_thickness_mm", units : "mm",      isWasmArg : true,  getter : getInt,       setter : setInt      }
  , { id : "flow_resistivity",      units : "rayls/m", isWasmArg : true,  getter : getInt,       setter : setInt      }
  , { id : "air_gap_mm",            units : "mm",      isWasmArg : true,  getter : getInt,       setter : setInt      }
  , { id : "cavity_depth_mm",       units : "mm",      isWasmArg : false, getter : getInt,       setter : setInt      }
  , { id : "graph_start_freq",      units : "Hz",      isWasmArg : true,  getter : getFloat,     setter : setFloat    }
  , { id : "smooth_curve",          units : "each",    isWasmArg : true,  getter : getCheckbox,  setter : setCheckbox }
  , { id : "subdivision",           units : "each",    isWasmArg : true,  getter : getRadio,     setter : setRadio    }
  ]
, "perforated_panel" : [
    { id : "panel_thickness_mm",    units : "mm",      isWasmArg : true,  getter : getFloat,     setter : setFloat    }
  , { id : "repeat_distance_mm",    units : "mm",      isWasmArg : true,  getter : getFloat,     setter : setFloat    }
  , { id : "hole_radius_mm",        units : "mm",      isWasmArg : true,  getter : getFloat,     setter : setFloat    }
  , { id : "porosity",              units : "each",    isWasmArg : true,  getter : getInnerHTML, setter : setFloat    }
  , { id : "absorber_thickness_mm", units : "mm",      isWasmArg : true,  getter : getInt,       setter : setInt      }
  , { id : "flow_resistivity",      units : "rayls/m", isWasmArg : true,  getter : getInt,       setter : setInt      }
  , { id : "air_gap_mm",            units : "mm",      isWasmArg : true,  getter : getInt,       setter : setInt      }
  , { id : "cavity_depth_mm",       units : "mm",      isWasmArg : false, getter : getInt,       setter : setInt      }
  , { id : "graph_start_freq",      units : "Hz",      isWasmArg : true,  getter : getFloat,     setter : setFloat    }
  , { id : "smooth_curve",          units : "each",    isWasmArg : true,  getter : getCheckbox,  setter : setCheckbox }
  , { id : "subdivision",           units : "each",    isWasmArg : true,  getter : getRadio,     setter : setRadio    }
  ]
, "microperforated_panel" : [
    { id : "panel_thickness_mm", units : "mm",      isWasmArg : true, getter : getFloat,     setter : setFloat    }
  , { id : "repeat_distance_mm", units : "mm",      isWasmArg : true, getter : getFloat,     setter : setFloat    }
  , { id : "hole_radius_mm",     units : "mm",      isWasmArg : true, getter : getFloat,     setter : setFloat    }
  , { id : "porosity",           units : "each",    isWasmArg : true, getter : getInnerHTML, setter : setFloat    }
  , { id : "air_gap_mm",         units : "mm",      isWasmArg : true, getter : getInt,       setter : setInt      }
  , { id : "angle",              units : "°",       isWasmArg : true, getter : getInt,       setter : setInt      }
  , { id : "graph_start_freq",   units : "Hz",      isWasmArg : true, getter : getFloat,     setter : setFloat    }
  , { id : "smooth_curve",       units : "each",    isWasmArg : true, getter : getCheckbox,  setter : setCheckbox }
  , { id : "subdivision",        units : "each",    isWasmArg : true, getter : getRadio,     setter : setRadio    }
  ]
  // Define default value properties only for the configuration fields.
  // If these are missing, then when the app starts for the first time, it attempts to read these missing values.
  // Since they are missing, they acquire a value of NaN...
  // If you then try to set an HTML range slider to NaN, it won't report any errors and it will take a best guess and
  // set the slider to its mid-point value - which results in the air temperature magically becoming 40°C and the air
  // pressure magically becoming 0.95 Bar...
, "configuration" : [
    { id : "air_temp",     units : "°C",  isWasmArg : true, getter : getInt,   setter : setInt,   default : "20"  }
  , { id : "air_pressure", units : "bar", isWasmArg : true, getter : getFloat, setter : setFloat, default : "1.0" }
  ]
}
 
// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************
export { tabConfig }

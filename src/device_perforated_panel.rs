// *********************************************************************************************************************
// Porous Absorber Calculator
//
// Perforated Panel Absorption Device
// 
// (c) Chris Whealy 2019
// *********************************************************************************************************************
use wasm_bindgen::JsValue;

use crate::structs::config_air::{AirConfig, AirError};
use crate::structs::config_cavity::{CavityConfig, CavityError};
use crate::structs::config_display::{DisplayConfig, DisplayError};
use crate::structs::config_porous_layer::{PorousLayerConfig, PorousLayerError};

use crate::structs::panel_perforated::{PerforatedPanelConfig, PerforatedPanelError};

use crate::calc_engine;
use crate::render;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Trace functionality
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
use crate::Trace;

const LIB_NAME     : &str  = &"device_perforated_panel";
const TRACE_ACTIVE : &bool = &false;

const CHART_TITLE : &str = &"Normal Incidence Absorption";

// *********************************************************************************************************************
// *********************************************************************************************************************
//
//                                                  P U B L I C   A P I
//
// *********************************************************************************************************************
// *********************************************************************************************************************



/***********************************************************************************************************************
 * Handle incoming arguments for calculating the absorption of a perforated panel absorption device
 */
pub fn do_perforated_panel_device(wasm_arg_obj : JsValue) -> JsValue {
  const FN_NAME : &str = &"do_perforated_panel_device";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  // Parse object received from JavaScript
  let arg_obj : PerforatedPanelArgs = wasm_arg_obj.into_serde().unwrap();

  // What values did we receive from JavaScript?
  trace(&format!("panel_thickness_mm    = {}", arg_obj.panel_thickness_mm));
  trace(&format!("repeat_distance_mm    = {}", arg_obj.repeat_distance_mm));
  trace(&format!("hole_radius_mm        = {}", arg_obj.hole_radius_mm));
  trace(&format!("porosity              = {}", arg_obj.porosity));
  trace(&format!("absorber_thickness_mm = {}", arg_obj.absorber_thickness_mm));
  trace(&format!("flow_resistivity      = {}", arg_obj.flow_resistivity));
  trace(&format!("air_gap_mm            = {}", arg_obj.air_gap_mm));
  trace(&format!("graph_start_freq      = {}", arg_obj.graph_start_freq));
  trace(&format!("smooth_curve          = {}", arg_obj.smooth_curve));
  trace(&format!("subdivision           = {}", arg_obj.subdivision));
  trace(&format!("show_diagram          = {}", arg_obj.show_diagram));
  trace(&format!("air_temp              = {}", arg_obj.air_temp));
  trace(&format!("air_pressure          = {}", arg_obj.air_pressure));

  // Parse arguments to the required data types
  let panel_thickness_mm    : f64  = arg_obj.panel_thickness_mm.parse().unwrap();
  let repeat_distance_mm    : f64  = arg_obj.repeat_distance_mm.parse().unwrap();
  let hole_radius_mm        : f64  = arg_obj.hole_radius_mm.parse().unwrap();
  let porosity              : f64  = arg_obj.porosity.parse().unwrap();
  let absorber_thickness_mm : u16  = arg_obj.absorber_thickness_mm.parse().unwrap();
  let flow_resistivity      : u32  = arg_obj.flow_resistivity.parse().unwrap();
  let air_gap_mm            : u16  = arg_obj.air_gap_mm.parse().unwrap();
  let graph_start_freq      : f64  = arg_obj.graph_start_freq.parse().unwrap();
  let smooth_curve          : bool = arg_obj.smooth_curve.parse().unwrap();
  let subdivision           : u16  = arg_obj.subdivision.parse().unwrap();
  let show_diagram          : bool = arg_obj.show_diagram.parse().unwrap();
  let air_temp              : i16  = arg_obj.air_temp.parse().unwrap();
  let air_pressure          : f64  = arg_obj.air_pressure.parse().unwrap();

  // Empty return data structure
  let mut error_msgs: Vec<String> = vec!();

  // Construct configuration structs
  let air_cfg = AirConfig::new(air_temp, air_pressure)
    .unwrap_or_else(|err: AirError| {
      error_msgs.push(String::from(err.to_string()));
      AirConfig::default()
    });

  let cavity_cfg = CavityConfig::new(air_gap_mm)
    .unwrap_or_else(|err: CavityError| {
      error_msgs.push(String::from(err.to_string()));
      CavityConfig::default()
    });

  let display_cfg = DisplayConfig::new(graph_start_freq, smooth_curve, subdivision, show_diagram)
    .unwrap_or_else(|err: DisplayError| {
      error_msgs.push(String::from(err.to_string()));
      DisplayConfig::default()
    });

  let panel_cfg = PerforatedPanelConfig::new(panel_thickness_mm, repeat_distance_mm, hole_radius_mm, porosity)
    .unwrap_or_else(|err: PerforatedPanelError| {
      error_msgs.push(String::from(err.to_string()));
      PerforatedPanelConfig::default()
    });

  let porous_cfg = PorousLayerConfig::new(absorber_thickness_mm, flow_resistivity)
    .unwrap_or_else(|err: PorousLayerError| {
      error_msgs.push(String::from(err.to_string()));
      PorousLayerConfig::default()
    });

  // If there are no error messages, then calculate the absorption values, plot the graph and return the placeholder
  // value "Ok", else return the array of error messages
  let return_value = if error_msgs.len() == 0 {
    let absorber_info = calc_engine::calculate_perforated_panel(
      &air_cfg
    , &cavity_cfg
    , &display_cfg
    , &panel_cfg
    , &porous_cfg
    );
    
    // Plot the graph
    let chart_info = render::plot_generic_device(absorber_info, &display_cfg, CHART_TITLE);

    JsValue::from_serde(&chart_info).unwrap()
  }
  else {
    // Serialize the error message(s)
    JsValue::from_serde(&error_msgs).unwrap()
  };

  trace_boundary(&Some(false));

  // Return either the {X,Y} values of plot points or the error messages back to JavaScript
  return_value
}


// *********************************************************************************************************************
// *********************************************************************************************************************
//
//                                                 P R I V A T E   A P I
//
// *********************************************************************************************************************
// *********************************************************************************************************************



/***********************************************************************************************************************
 * Arguments required by function do_perforated_panel_device
 */
#[derive(Deserialize)]
struct PerforatedPanelArgs {
  panel_thickness_mm    : String    // Internally treated as f64
, repeat_distance_mm    : String    // Internally treated as f64
, hole_radius_mm        : String    // Internally treated as f64
, porosity              : String    // Internally treated as f64
, absorber_thickness_mm : String    // Internally treated as u16
, flow_resistivity      : String    // Internally treated as u32
, air_gap_mm            : String    // Internally treated as u16
, graph_start_freq      : String    // Internally treated as f64
, smooth_curve          : String    // Internally treated as bool
, subdivision           : String    // Internally treated as u16
, show_diagram          : String    // Internally treated as bool
, air_temp              : String    // Internally treated as i16
, air_pressure          : String    // Internally treated as f64
}

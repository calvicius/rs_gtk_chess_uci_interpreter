extern crate gio;
extern crate gtk;
extern crate gdk;
extern crate glib;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;


mod uci_interpreter;


fn main() {
    let status: i32;
    
    let app = gtk::Application::new (Some("org.anonimo.autor"), gio::ApplicationFlags::FLAGS_NONE).unwrap();
    app.connect_activate(|widget| {
        // We build the application UI.
        activar(widget);
    });
    status = app.run(&args().collect::<Vec<_>>());
    eprintln!("Exit code: {}", status);
}


fn activar (application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_border_width(5);
    window.set_default_size(800, 350);
    
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.add(&vbox);
    
    let mut engine = uci_interpreter::Engine::init("./engine/stockfish.exe");
    vbox.pack_start(&engine.scrolled_win, true, true, 0);
    
    // false space
    let lbl = gtk::Label::new(Some(""));
    vbox.pack_start(&lbl, false, false, 0);
    
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    vbox.pack_start(&hbox, false, false, 0);
    
    let lbl_depth = gtk::Label::new(Some("Calculation depth: "));
    hbox.pack_start(&lbl_depth, false, false, 0);
    let spin_depth = gtk::SpinButton::new_with_range(5.0, 100.0, 5.0);
    spin_depth.set_wrap(true);
    hbox.pack_start(&spin_depth, false, false, 0);
    
    let lbl_fen = gtk::Label::new(Some("  Select FEN: "));
    hbox.pack_start(&lbl_fen, false, false, 0);
    let combo = gtk::ComboBoxText::new();
    combo.append_text("rnbqk2r/pppp1ppp/4pn2/8/1bPP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 4");
    combo.append_text("r2r2k1/pp3pbp/2p1b1p1/2Pn1p2/P2P4/3BP1P1/1B1N1P1P/2R2RK1 w - - 0 21");
    combo.append_text("r5k1/p1r2pbp/1pp1b1p1/2Pn1p2/P2P4/1N2P1P1/4BP1P/BR3RK1 b - - 0 26");
    combo.append_text("r1b1r1k1/5q1p/p1p1p1pQ/1pP1Rp2/3P4/P7/BP3PPP/4R1K1 w - - 0 28");
    combo.append_text("r2q1b1r/pp1b2pp/2p5/2kNP3/8/3Q4/PPP2PPP/R1B3K1 w - - 0 17");
    combo.append_text("rnb4r/pppp2pp/7k/6Qn/3PP2B/2N5/PPP4P/R3K3 b Q - 0 15");
    combo.append_text("r1bk1r2/ppp1n2p/1b1p3Q/6B1/2B2P2/8/P5PP/qN3R1K w - - 0 18");
    hbox.pack_start(&combo, false, false, 0);
    
    let btn_depth = gtk::Button::new_with_label("Analysis sel. FEN and given depth");
    hbox.pack_start(&btn_depth, false, false, 0);
    
    let btn_infinite = gtk::Button::new_with_label("Analysis infinite");
    hbox.pack_start(&btn_infinite, false, false, 0);
    
    let btn_time = gtk::Button::new_with_label("Analysis 500 ms");
    hbox.pack_start(&btn_time, false, false, 0);
    
    window.show_all();
    
    engine.get_handshake();
    engine.get_uci_options();
    // an example to change default engine options
    /* engine.set_options(b"setoption name MultiPV value 10\n"); */
    
    let is_ready_ok = engine.get_is_ready();
    if is_ready_ok.contains("readyok") {
      engine.set_initial_pos();
      engine.go_depth(spin_depth.get_value_as_int());
      
      let movs = "e2e4 c7c5";
      engine.set_initial_pos_with_moves(movs);
      engine.go_depth(spin_depth.get_value_as_int());
      //engine.go_infinite();
    }
    
    let engine1 = engine.clone();
    let engine2 = engine.clone();
    let engine3 = engine.clone();
    let engine4 = engine.clone();
    {
      // Activate ESC key
      window.connect_key_press_event ( move |_widget, event| {
        let engine_clon = engine1.clone();
        if event.get_keyval() == gdk::enums::key::Escape {
          engine_clon.set_stop();
          //println!("////////////////////////////////////////////////");
        }
        Inhibit(false)
      });
    }
    
    {
      let combo_clon = combo.clone();
      let ready_ok_clon = is_ready_ok.clone();
      btn_depth.connect_clicked (move |_w| {
        let mut engine_clon = engine2.clone();
        if ready_ok_clon.contains("readyok") {
          let fen = combo_clon.get_active_text();
          match fen {
            Some(gfen) => {
              let fen1 = gfen.as_str();
              engine_clon.set_pos_fen (fen1);
              engine_clon.go_depth(spin_depth.get_value_as_int());
            },
            None => {
              // initial position
              let fen1 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
              engine_clon.set_pos_fen (fen1);
              engine_clon.go_depth(spin_depth.get_value_as_int());
            }
          }
        }
      });
    }
    
    {
      let combo_clon = combo.clone();
      let ready_ok_clon = is_ready_ok.clone();
      btn_infinite.connect_clicked (move |_w| {
        let mut engine_clon = engine3.clone();
        if ready_ok_clon.contains("readyok") {
          let fen = combo_clon.clone().get_active_text();
          match fen {
            Some(gfen) => {
              let fen1 = gfen.as_str();
              engine_clon.set_pos_fen (fen1);
              engine_clon.go_infinite();
            },
            None => {
              // initial position
              let fen1 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
              engine_clon.set_pos_fen (fen1);
              engine_clon.go_infinite();
            }
          }
        }
      });
    }
    
    {
      let combo_clon = combo.clone();
      let ready_ok_clon = is_ready_ok.clone();
      btn_time.connect_clicked (move |_w| {
        let mut engine_clon = engine4.clone();
        if ready_ok_clon.contains("readyok") {
          let fen = combo_clon.clone().get_active_text();
          match fen {
            Some(gfen) => {
              let fen1 = gfen.as_str();
              engine_clon.set_pos_fen (fen1);
              engine_clon.go_by_time(500);
            },
            None => {
              // initial position
              let fen1 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
              engine_clon.set_pos_fen (fen1);
              engine_clon.go_by_time(500);
            }
          }
        }
      });
    }
}



extern crate glk;
extern crate glkterm;

use glkterm::GlkTerm;
use glk::{Glk,EventType,IdType};

// model.c: Model program for Glk API, version 0.5.
//  Designed by Andrew Plotkin <erkyrath@eblong.com>
//  http://www.eblong.com/zarf/glk/index.html
//  This program is in the public domain.
//
// This is a simple model of a text adventure which uses the Glk API.
//  It shows how to input a line of text, display results, maintain a
//  status window, write to a transcript file, and so on.

fn main() {
    glkterm::init(glk_main);
}

fn glk_main(glk: GlkTerm, _args: Vec<String>) {
    Model::new(glk).glk_main();
}

pub struct Model<'a,G: Glk<'a>> {
    glk: G,

    // The story, status, and quote windows.
    mainwin: G::WinId,
    statuswin: G::WinId,
    quotewin: G::WinId,

    // A file reference for the transcript file.
    scriptref: G::FRefId,
    // A stream for the transcript file, when it's open.
    scriptstr: G::StrId,

    // Your location. This determines what appears in the status line.
    current_room: i32,

    // A flag indicating whether you should look around.
    need_look: bool,

    wcount1: usize,
    wcount2: usize,
    wstep: usize,
    jx: usize,
}

impl<'a,G: Glk<'a>> Model<'a,G> {
    pub fn new(glk: G) -> Self {
        Model{
            glk: glk,
            mainwin: G::WinId::null(),
            statuswin: G::WinId::null(),
            quotewin: G::WinId::null(),
            scriptref: G::FRefId::null(),
            scriptstr: G::StrId::null(),
            current_room: 0,
            need_look: false,

            wcount1: 0,
            wcount2: 0,
            wstep: 1,
            jx: 0,
        }
    }

    pub fn glk_main(mut self) {
        // Open the main window.
        self.mainwin = self.glk.window_open(&G::WinId::null(), 0, 0, glk::wintype_TextBuffer, 1);
        if self.mainwin.is_null() {
            // It's possible that the main window failed to open. There's
            //  nothing we can do without it, so exit.
            return;
        }

        // Set the current output stream to print to it.
        self.glk.set_window(&self.mainwin);

        // Open a second window: a text grid, above the main window, three lines
        //  high. It is possible that this will fail also, but we accept that.
        self.statuswin = self.glk.window_open(&self.mainwin, glk::winmethod_Above | glk::winmethod_Fixed, 3, glk::wintype_TextGrid, 0);

        // The third window, quotewin, isn't opened immediately. We'll do
        //  that in verb_quote().

        self.glk.put_string("Model Glk Program\nAn Interactive Model Glk Program\n");
        self.glk.put_string("By Andrew Plotkin.\nRelease 7.\n");
        self.glk.put_string("Type \"help\" for a list of commands.\n");

        self.current_room = 0; // Set initial location.
        self.need_look = true;

        let mut commandbuf = Some((0,vec![0u8; 256].into_boxed_slice()));
        loop {
            self.draw_statuswin();

            if self.need_look {
                self.need_look = false;
                self.glk.put_string("\n");
                self.glk.set_style(glk::style_Subheader);
                if self.current_room == 0 {
                    self.glk.put_string("The Room\n");
                } else {
                    self.glk.put_string("A Different Room\n");
                }
                self.glk.set_style(glk::style_Normal);
                self.glk.put_string("You're in a room of some sort.\n");
            }

            self.glk.put_string("\n>");
            // We request up to 255 characters. The buffer can hold 256, but we
            //  are going to stick a null character at the end, so we have to
            //  leave room for that. Note that the Glk library does *not*
            //  put on that null character.
            self.glk.request_line_event(&self.mainwin, commandbuf.take().unwrap(), 0);

            let mut command_len = 0;
            while commandbuf.is_none() {
                // Grab an event.
                let mut ev = self.glk.select();
                match ev.evtype() {
                    glk::evtype_LineInput => {
                        if ev.win() == self.mainwin {
                            // Really the event can *only* be from mainwin,
                            //  because we never request line input from the
                            //  status window. But we do a paranoia test,
                            //  because commandbuf is only filled if the line
                            //  event comes from the mainwin request. If the
                            //  line event comes from anywhere else, we ignore
                            //  it.
                            commandbuf = ev.buf();
                            command_len = ev.val1() as usize;
                        }
                    },
                    glk::evtype_Arrange => {
                        self.draw_statuswin();
                    },
                    _ => (),
                }
            }

            // commandbuf now contains a line of input from the main window.
            //  You would now run your parser and do something with it.

            // First, if there's a blockquote window open, let's close it.
            //  This ensures that quotes remain visible for exactly one
            //  command.
            if !self.quotewin.is_null() {
                self.glk.window_close(&mut self.quotewin);
            }

            // The line we have received in commandbuf is not null-terminated.
            //  We handle that first.

            // Then squash to lower-case.
            let mut buf = commandbuf.take().unwrap().1;
            for i in 0 .. command_len {
                buf[i] = self.glk.char_to_lower(buf[i]);
            }

            // Then trim whitespace before and after.
            {
                let cmd = std::str::from_utf8(&buf[0 .. command_len]).unwrap().trim();
                // We'll do the simplest possible parsing.
                match cmd {
                    "" => self.glk.put_string("Excuse me?\n"),
                    "help" => self.verb_help(),
                    "move" => self.verb_move(),
                    "jump" => self.verb_jump(),
                    "yada" => self.verb_yada(),
                    "quote" => self.verb_quote(),
                    "quit" => self.verb_quit(),
                    "save" => self.verb_save(),
                    "restore" => self.verb_restore(),
                    "script" => self.verb_script(),
                    "unscript" => self.verb_unscript(),
                    _ => {
                        self.glk.put_string("I don't understand the command \"");
                        self.glk.put_string(cmd);
                        self.glk.put_string("\".\n");
                    },

                }
            }
            commandbuf = Some((0,buf));
        }
    }

    fn draw_statuswin(&mut self) {
        if self.statuswin.is_null() {
            // It is possible that the window was not successfully 
            //  created. If that's the case, don't try to draw it.
            return;
        }

        let roomname = if self.current_room == 0 {
            "The Room"
        } else {
            "A Different Room"
        };

        self.glk.set_window(&self.statuswin);
        self.glk.window_clear(&self.statuswin);

        let (width,_) = self.glk.window_get_size(&self.statuswin);

        // Print the room name, centered.
        self.glk.window_move_cursor(&self.statuswin, (width - roomname.len() as u32) / 2, 1);
        self.glk.put_string(roomname);

        // Draw a decorative compass rose in the upper right.
        self.glk.window_move_cursor(&self.statuswin, width - 3, 0);
        self.glk.put_string("\\|/");
        self.glk.window_move_cursor(&self.statuswin, width - 3, 1);
        self.glk.put_string("-*-");
        self.glk.window_move_cursor(&self.statuswin, width - 3, 2);
        self.glk.put_string("/|\\");
    
        self.glk.set_window(&self.mainwin);
    }

    fn yes_or_no(&mut self) -> bool {
        self.draw_statuswin();

        // This loop is identical to the main command loop in glk_main().
        let mut commandbuf = Some((0,vec![0u8; 256].into_boxed_slice()));
        loop {
            self.glk.request_line_event(&self.mainwin, commandbuf.take().unwrap(), 0);

            let mut command_len = 0;
            while commandbuf.is_none() {

                let mut ev = self.glk.select();

                match ev.evtype() {
                    glk::evtype_LineInput => {
                        if ev.win() == self.mainwin {
                            commandbuf = ev.buf();
                            command_len = ev.val1() as usize;
                        }
                    },
                    glk::evtype_Arrange => {
                        self.draw_statuswin();
                    },
                    _ => (),
                }
            }
        
            // commandbuf now contains a line of input from the main window.
            //  You would now run your parser and do something with it.
        
            // First, if there's a blockquote window open, let's close it. 
            //  This ensures that quotes remain visible for exactly one
            //  command.
            if !self.quotewin.is_null() {
                self.glk.window_close(&mut self.quotewin);
            }
        
            // Then trim whitespace before and after.
            let buf = commandbuf.take().unwrap().1;
            {
                let cmd = std::str::from_utf8(&buf[0 .. command_len]).unwrap().trim();

                if cmd.starts_with("y") || cmd.starts_with("Y") {
                    return true;
                }
                if cmd.starts_with("n") || cmd.starts_with("N") {
                    return false;
                }

                self.glk.put_string("Please enter \"yes\" or \"no\": ");
            }
            commandbuf = Some((0,buf));
        }
    }

    fn verb_help(&mut self) {
        self.glk.put_string("This model only understands the following commands:\n");
        self.glk.put_string("HELP: Display this list.\n");
        self.glk.put_string("JUMP: A verb which just prints some text.\n");
        self.glk.put_string("YADA: A verb which prints a very long stream of text.\n");
        self.glk.put_string("MOVE: A verb which prints some text, and also changes the status line display.\n");
        self.glk.put_string("QUOTE: A verb which displays a block quote in a temporary third window.\n");
        self.glk.put_string("SCRIPT: Turn on transcripting, so that output will be echoed to a text file.\n");
        self.glk.put_string("UNSCRIPT: Turn off transcripting.\n");
        self.glk.put_string("SAVE: Write fake data to a save file.\n");
        self.glk.put_string("RESTORE: Read it back in.\n");
        self.glk.put_string("QUIT: Quit and exit.\n");
    }

    fn verb_jump(&mut self) {
        self.glk.put_string("You jump on the fruit, spotlessly.\n");
    }

    fn verb_yada(&mut self) {
        // This is a goofy (and overly ornate) way to print a long paragraph. 
        //  It just shows off line wrapping in the Glk implementation.
        let wordcaplist = [
            "Ga", "Bo", "Wa", "Mu", "Bi", "Fo", "Za", "Mo", "Ra", "Po",
            "Ha", "Ni", "Na"
                ];
        let wordlist = [
            "figgle", "wob", "shim", "fleb", "moobosh", "fonk", "wabble",
            "gazoon", "ting", "floo", "zonk", "loof", "lob",
            ];

        let mut first = true;
        for ix in 0 .. 85 {
            if ix > 0 {
                self.glk.put_string(" ");
            }

            if first {
                self.glk.put_string(wordcaplist[(ix/17) % wordcaplist.len()]);
                first = false;
            }

            self.glk.put_string(wordlist[self.jx]);
            self.jx = (self.jx + self.wstep) % wordlist.len();
            self.wcount1 += 1;
            if self.wcount1 >= wordlist.len() {
                self.wcount1 = 0;
                self.wstep += 1;
                self.wcount2 += 1;
                if self.wcount2 >= wordlist.len()-2 {
                    self.wcount2 = 0;
                    self.wstep = 1;
                }
            }

            if ix % 17 == 16 {
                self.glk.put_string(".");
                first = true;
            }
        }
        self.glk.put_char(b'\n');
    }

    fn verb_quote(&mut self) {
        self.glk.put_string("Someone quotes some poetry.\n");

        // Open a third window, or clear it if it's already open. Actually,
        //  since quotewin is closed right after line input, we know it
        //  can't be open. But better safe, etc.
        if self.quotewin.is_null() {
            // A five-line window above the main window, fixed size.
            self.quotewin = self.glk.window_open(&self.mainwin, glk::winmethod_Above | glk::winmethod_Fixed, 5, glk::wintype_TextBuffer, 0);
            if self.quotewin.is_null() {
                // It's possible the quotewin couldn't be opened. In that
                //  case, just give up.
                return;
            }
        } else {
            self.glk.window_clear(&self.quotewin);
        }

        // Print some quote.
        self.glk.set_window(&self.quotewin);
        self.glk.set_style(glk::style_BlockQuote);
        self.glk.put_string("Tomorrow probably never rose or set\nOr went out and bought cheese, or anything like that\nAnd anyway, what light through yonder quote box breaks\nHandle to my hand?\n");
        self.glk.put_string("              -- Fred\n");

        self.glk.set_window(&self.mainwin);
    }

    fn verb_move(&mut self) {
        self.current_room = (self.current_room+1) % 2;
        self.need_look = true;

        self.glk.put_string("You walk for a while.\n");
    }

    fn verb_quit(&mut self) {
        self.glk.put_string("Are you sure you want to quit? ");
        if self.yes_or_no() {
            self.glk.put_string("Thanks for playing.\n");
            self.glk.exit();
            // glk_exit() actually stops the process; it does not return.
        }
    }

    fn verb_script(&mut self) {
        if !self.scriptstr.is_null() {
            self.glk.put_string("Scripting is already on.\n");
            return;
        }

        // If we've turned on scripting before, use the same file reference;
        //  otherwise, prompt the player for a file.
        if self.scriptref.is_null() {
            self.scriptref = self.glk.fileref_create_by_prompt(glk::fileusage_Transcript | glk::fileusage_TextMode, glk::filemode_WriteAppend, 0);
            if self.scriptref.is_null() {
                self.glk.put_string("Unable to place script file.\n");
                return;
            }
        }

        // Open the file.
        self.scriptstr = self.glk.stream_open_file(&self.scriptref, glk::filemode_WriteAppend, 0);
        if self.scriptstr.is_null() {
            self.glk.put_string("Unable to write to script file.\n");
            return;
        }
        self.glk.put_string("Scripting on.\n");
        self.glk.window_set_echo_stream(&self.mainwin, &self.scriptstr);
        self.glk.put_string_stream(&self.scriptstr, "This is the beginning of a transcript.\n");
    }

    fn verb_unscript(&mut self) {
        if self.scriptstr.is_null() {
            self.glk.put_string("Scripting is already off.\n");
            return;
        }

        // Close the file.
        self.glk.put_string_stream(&self.scriptstr, "This is the end of a transcript.\n\n");
        self.glk.stream_close(&mut self.scriptstr);
        self.glk.put_string("Scripting off.\n");
    }

    fn verb_save(&mut self) {
        let mut saveref = self.glk.fileref_create_by_prompt(glk::fileusage_SavedGame | glk::fileusage_BinaryMode, glk::filemode_Write, 0);
        if saveref.is_null() {
            self.glk.put_string("Unable to place save file.\n");
            return;
        }

        let mut savestr = self.glk.stream_open_file(&saveref, glk::filemode_Write, 0);
        if savestr.is_null() {
            self.glk.put_string("Unable to write to save file.\n");
            self.glk.fileref_destroy(&mut saveref);
            return;
        }

        self.glk.fileref_destroy(&mut saveref); // We're done with the file ref now.

        // Write some binary data.
        for ix in 0u32 .. 256 {
            self.glk.put_char_stream(&savestr, ix as u8);
        }

        self.glk.stream_close(&mut savestr);

        self.glk.put_string("Game saved.\n");
    }

    fn verb_restore(&mut self) {
        let mut saveref = self.glk.fileref_create_by_prompt(glk::fileusage_SavedGame | glk::fileusage_BinaryMode, glk::filemode_Read, 0);
        if saveref.is_null() {
            self.glk.put_string("Unable to find save file.\n");
            return;
        }

        let mut savestr = self.glk.stream_open_file(&saveref, glk::filemode_Read, 0);
        if savestr.is_null() {
            self.glk.put_string("Unable to read from save file.\n");
            self.glk.fileref_destroy(&mut saveref);
            return;
        }

        self.glk.fileref_destroy(&mut saveref); // We're done with the file ref now.

        // Read some binary data.
        let mut err = false;

        for ix in 0 .. 256 {
            let ch = self.glk.get_char_stream(&savestr);
            if ch == -1 {
                self.glk.put_string("Unexpected end of file.\n");
                err = true;
                break;
            }
            if ch != ix {
                self.glk.put_string("This does not appear to be a valid saved game.\n");
                err = true;
                break;
            }
        }

        self.glk.stream_close(&mut savestr);

        if err {
            self.glk.put_string("Failed.\n");
            return;
        }

        self.glk.put_string("Game restored.\n");
    }
}

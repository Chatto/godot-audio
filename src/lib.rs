extern crate gdnative;
use gdnative::*;

/// The HelloWorld "class"
#[derive(NativeClass)]
#[inherit(AudioStreamPlayer)]
pub struct PlayAudio;

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl PlayAudio {
    
    /// The "constructor" of the class.
    fn _init(_owner:AudioStreamPlayer) -> Self {
        PlayAudio
    }
    
    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are"attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    unsafe fn _ready(&self, mut _owner:AudioStreamPlayer) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("Greetings Bolster!");
        godot_print!("Attempting to play audio...");
        AudioStreamPlayer::play(&mut _owner, 0.0);
        godot_print!("Playing audio...");
        godot_print!("Setting up loop...");
        let object = &_owner.to_object();
        _owner
        .connect(
            GodotString::from_str("finished"),
            Some(*object),
            GodotString::from_str("_finished"),
            VariantArray::new(),
            1,
        )
        .unwrap();

        //_owner.emit_signal(GodotString::from_str("finished"), &[]);

        godot_print!("Loop Set...");
    
    }

    #[export]
    unsafe fn _finished(&mut self, mut _owner:AudioStreamPlayer){
    AudioStreamPlayer::play(&mut _owner, 0.0);
    godot_print!("Audio has looped!");
}
}

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<PlayAudio>();
}


// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();

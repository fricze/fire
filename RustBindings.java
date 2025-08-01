import java.lang.foreign.*;
import java.lang.foreign.MemoryLayout.PathElement;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.VarHandle;

public class RustBindings {

    static MethodHandle addNumbers; // Wrapper for the Rust function
    static MethodHandle giveString; // Wrapper for the Rust function
    static VarHandle strHandle;
    static VarHandle byteLenHandle;

    static {
        // Initialize the linker
        Linker linker = Linker.nativeLinker();

        // Load the Rust library
        SymbolLookup lib = SymbolLookup.libraryLookup(
            "target/release/libfire.dylib",
            Arena.global()
        );

        // Link the Rust function
        addNumbers = linker.downcallHandle(
            lib.find("add_numbers").orElseThrow(), // Replace with the function name from Rust
            FunctionDescriptor.of(
                ValueLayout.JAVA_INT, // Rust's return type: i32
                ValueLayout.JAVA_INT, // Rust's first parameter: i32
                ValueLayout.JAVA_INT // Rust's second parameter: i32
            )
        );

        StructLayout pointLayout = MemoryLayout.structLayout(
            ValueLayout.ADDRESS.withName("str"), // Maps to Rust's i32 `x`
            ValueLayout.JAVA_INT.withName("byte_len"), // Maps to Rust's i32 `y`
            MemoryLayout.paddingLayout(4) // 4 bytes of padding
        );

        strHandle = pointLayout.varHandle(PathElement.groupElement("str"));
        byteLenHandle = pointLayout.varHandle(
            PathElement.groupElement("byte_len")
        );

        giveString = linker.downcallHandle(
            lib.find("give_string").orElseThrow(),
            FunctionDescriptor.of(pointLayout)
        );
    }
}

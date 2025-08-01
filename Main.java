import java.lang.foreign.Arena; // For allocating memory if needed
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SegmentAllocator;

public class Main {

    public static void main(String[] args) throws Throwable {
        int result = (int) RustBindings.addNumbers.invokeExact(10, 20);
        System.out.println("Result: " + result); // Output should be 30

        try (Arena arena = Arena.ofConfined()) {
            SegmentAllocator allocator = arena;

            // Invoke the handle, passing the allocator as the first argument.
            MemorySegment stringStruct =
                (MemorySegment) RustBindings.giveString.invokeExact(allocator);

            MemorySegment strAddress =
                (MemorySegment) RustBindings.strHandle.get(stringStruct, 0L);
            int byteLen = (int) RustBindings.byteLenHandle.get(
                stringStruct,
                0L
            );

            if (strAddress != MemorySegment.NULL) {
                MemorySegment sizedStringPtr = strAddress.reinterpret(byteLen);
                String rustString = sizedStringPtr.getString(0); // This will now work
                System.out.println("String from Rust: " + rustString);
            } else {
                System.out.println("Rust returned a null string pointer.");
            }
        }
    }
}

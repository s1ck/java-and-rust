import java.util.List;

class Main {
    // Declares a method that will be provided by a native library.
    private static native String hello(String input);
 
    // Passing down to Lists and returning a primitive.
    private static native long dotProduct(List<Long> vectorA, List<Long> vectorB);
    
    // Passing down to primitive arrays and returning a primitive.
    private static native long dotProductArray(long[] vectorA, long[] vectorB);

    // // Passing down to primitive arrays and returning a primitive.
    // // We try to get those arrays in critical mode, i.e., trying to avoid a copy.
    // private static native long dotProductCritical(long[] vectorA, long[] vectorB);

    // // Passing down primitive arrays and a callback method.
    // private static native void dotProductConsume(long[] vectorA, long[] vectorB, Callback callback);
    
    // // Instatiate a new instance of "Counter" in native code.
    // // The method returns the address of the newly created object.
    // private static native long counterNew(Callback callback);
    // // Calls a method on the Counter object in native code
    // private static native void counterInc(long counterPtr);
    // // Destroys the counter object.
    // private static native void counterDes(long counterPtr);

    // // Runs a computation in a different thread in native code.
    // private static native void asyncComputation(Callback callback);

    static {
        // Load the shared object (.so on Linux, .dll on Windows)
        System.loadLibrary("mylib");
    }

    // @FunctionalInterface
    // public static interface Callback {
    //     void call(long value);
    // }

    public static void main(String[] args) {
        // Example 1: Hello World
        String greeting = Main.hello("Alice");
        System.out.println(greeting);
 
        // Example 2a: Pass two primitive arrays and get a result
        List<Long> vectorA = List.of(1L, 3L, 3L, 7L);
        List<Long> vectorB = List.of(1L, 9L, 8L, 4L);
        long dotProduct = Main.dotProduct(vectorA, vectorB);
        System.out.println("dot product = " + dotProduct);
        
        
        // Example 2b: Pass two primitive arrays and get a result
        long[] vectorA_Array = {1L, 3L, 3L, 7L};
        long[] vectorB_Array = {1L, 9L, 8L, 4L};
        dotProduct = Main.dotProductArray(vectorA_Array, vectorB_Array);
        System.out.println("dot product = " + dotProduct);
 
        // // Example 3: Pass two primitive arrays and a result consumer
        // Main.dotProductConsume(vectorA, vectorB, result -> System.out.println("dot product = " + result));
 
        // // Example 4: Instantiate an object in native code and interact with it
        // long counterPtr = Main.counterNew(count -> System.out.println("count = " + count));
        // Main.counterInc(counterPtr);
        // Main.counterInc(counterPtr);
        // // Segfault example
        // // Main.counterInc(1337);
        // Main.counterDes(counterPtr);

        // // Example 5: Invoking an async computation
        // System.out.println("Invoking computation (threadId = " + Thread.currentThread().getId() + ")");
        // Main.asyncComputation(progress -> System.out.println(
        //     "asyncCallback: thread id = " +
        //     Thread.currentThread().getId() +
        //     ", progress = " + progress + "%"
        // ));
    }
}

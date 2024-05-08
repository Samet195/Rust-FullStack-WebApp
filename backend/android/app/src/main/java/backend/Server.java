package backend;

public class Server {
    public static native int start();
    public static native void pause();
    public static native void resume();
    public static native void stop();

    static {
        System.loadLibrary("backend");
    }
}

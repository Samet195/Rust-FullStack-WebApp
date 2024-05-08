package engineer.sametarik.webapp;

import android.annotation.SuppressLint;
import android.app.Activity;
import android.os.Bundle;
import android.webkit.WebSettings;
import android.webkit.WebView;

public class MainActivity extends Activity {
    private Bundle state;
    private WebView view;

    @Override
    @SuppressLint("SetJavaScriptEnabled")
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        setContentView(R.layout.activity_main);
        view = findViewById(R.id.activity_main_webview);
        state = savedInstanceState;
        int port = backend.Server.start();

        WebSettings settings = view.getSettings();
        settings.setCacheMode(WebSettings.LOAD_NO_CACHE);
        settings.setDomStorageEnabled(true);
        settings.setJavaScriptEnabled(true);
        settings.setSupportZoom(false);

        view.setSaveEnabled(true);
        view.setSaveFromParentEnabled(true);
        view.setWebViewClient(new WebClient());

        view.loadUrl("https://127.0.0.1:" + port);
        view.restoreState(state);
    }

    @Override
    protected void onPause() {
        backend.Server.pause();
        view.onPause();
        view.saveState(state);
        super.onPause();
    }

    @Override
    protected void onResume() {
        super.onResume();
        backend.Server.resume();
        view.onResume();
        view.restoreState(state);
    }

    @Override
    protected void onDestroy() {
        view.saveState(state);
        super.onDestroy();
        backend.Server.stop();
    }

    @Override
    public void onBackPressed() {
        if (view.canGoBack()) {
            view.goBack();
        } else {
            super.onBackPressed();
        }
    }
}

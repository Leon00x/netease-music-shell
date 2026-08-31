// 网易云音乐 Web Player 桌面壳
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{image::Image, WebviewUrl, WebviewWindowBuilder};

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";

// WebKitGTK 未实现 requestIdleCallback，网易云 webplayer 的 Chrome 代码路径需要它
const POLYFILL: &str = r#"
if (typeof window.requestIdleCallback !== 'function') {
  window.requestIdleCallback = function(cb, opts) {
    return setTimeout(function(){ cb({ didTimeout: false, timeRemaining: function(){ return 50; } }); }, 1);
  };
  window.cancelIdleCallback = function(id){ clearTimeout(id); };
}
"#;

// 圆角窗口：用透明渐变占据根元素的 canvas 背景，阻止 body 背景传播成方形；
// 再让 body 成为 fixed 后代的 containing block，并在 body 上统一裁剪。
const ROUND_CORNERS: &str = r#"
(function(){
  function install(){
    if (!document.head) return false;
    if (document.getElementById('__nmcorner-style')) return true;
    var style = document.createElement('style');
    style.id = '__nmcorner-style';
    style.textContent = ''
      + 'html{background-color:transparent!important;'
      + 'background-image:linear-gradient(transparent,transparent)!important;'
      + 'overflow:hidden!important;}'
      + 'body{position:fixed!important;inset:0!important;margin:0!important;'
      + 'border-radius:16px!important;overflow:hidden!important;'
      + 'transform:translateZ(0)!important;}';
    document.head.appendChild(style);
    return true;
  }
  if (!install()) document.addEventListener('DOMContentLoaded', install, { once:true });
})();
"#;

// 全宽悬浮标题栏：鼠标靠近窗口顶部 ~30px 内浮现，整条可拖拽，右上角控制按钮
const WINDOW_CONTROLS: &str = r#"
(function(){
  if (window.__nm_wc) return; window.__nm_wc = true;
  var CSS = ''
    + '#nm-wc{position:fixed;top:0;left:0;right:0;height:38px;z-index:2147483646;display:flex;'
    + 'align-items:center;justify-content:flex-end;padding:0 10px;gap:2px;pointer-events:none;'
    + 'opacity:0;transition:opacity .16s ease;user-select:none;-webkit-user-select:none;'
    + 'background:linear-gradient(rgba(0,0,0,.50),rgba(0,0,0,.18));backdrop-filter:blur(10px);'
    + 'border-radius:16px 16px 0 0;'
    + 'font-family:system-ui,sans-serif;}'
    + '#nm-wc.on{pointer-events:auto;opacity:1;}'
    + '#nm-wc .nm-drag{flex:1;height:100%;}'
    + '#nm-wc button{all:unset;cursor:pointer;width:36px;height:28px;border-radius:7px;display:flex;'
    + 'align-items:center;justify-content:center;color:#fff;}'
    + '#nm-wc button:hover{background:rgba(255,255,255,.22);}'
    + '#nm-wc button.nm-close:hover{background:#e81123;}'
    + '#nm-wc svg{display:block;pointer-events:none;}';
  var MIN = '<svg width="11" height="11" viewBox="0 0 11 11"><path d="M1 5.5h9" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/></svg>';
  var MAX = '<svg width="10" height="10" viewBox="0 0 10 10"><rect x="1" y="1" width="8" height="8" rx="1.5" fill="none" stroke="currentColor" stroke-width="1.3"/></svg>';
  var CLOSE = '<svg width="11" height="11" viewBox="0 0 11 11"><path d="M1.5 1.5l8 8M9.5 1.5l-8 8" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/></svg>';

  function boot(){
    if (!window.__TAURI__ || !window.__TAURI__.window) return false;
    if (!document.body) return false;
    var w = window.__TAURI__.window.getCurrentWindow();

    var style = document.createElement('style');
    style.textContent = CSS;
    document.head.appendChild(style);

    var bar = document.createElement('div');
    bar.id = 'nm-wc';
    var drag = document.createElement('div');
    drag.className = 'nm-drag';
    drag.setAttribute('data-tauri-drag-region', '');
    bar.appendChild(drag);

    function mk(svg, cls, tip, fn){
      var b = document.createElement('button');
      b.className = cls; b.title = tip; b.innerHTML = svg;
      b.addEventListener('click', function(e){ e.stopPropagation(); e.preventDefault(); fn(); });
      return b;
    }
    bar.appendChild(mk(MIN,  'nm-min',   '最小化', function(){ w.minimize(); }));
    bar.appendChild(mk(MAX,  'nm-max',   '最大化/还原', function(){ w.toggleMaximize(); }));
    bar.appendChild(mk(CLOSE,'nm-close', '关闭', function(){ w.close(); }));

    var trigger = document.createElement('div');
    trigger.id = 'nm-wc-trigger';
    // 用 mousemove 检测：鼠标接近顶部 30px 即浮现，不遮挡下方页面元素的点击
    document.addEventListener('mousemove', function(e){
      if (e.clientY <= 30) bar.classList.add('on');
    }, { passive: true });
    bar.addEventListener('mouseleave', function(){ bar.classList.remove('on'); });

    document.body.appendChild(bar);
    return true;
  }
  var tries = 0;
  var timer = setInterval(function(){ if (boot() || ++tries > 80) clearInterval(timer); }, 100);
})();
"#;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let icon = Image::from_bytes(include_bytes!("../icons/icon.png"))?;
            WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://music.163.com/st/webplayer".parse()?),
            )
            .icon(icon)?
            .title("网易云音乐")
            .inner_size(1280.0, 860.0)
            .min_inner_size(960.0, 640.0)
            .center()
            .decorations(false)
            .transparent(true)
            .user_agent(USER_AGENT)
            .initialization_script(POLYFILL)
            .initialization_script(ROUND_CORNERS)
            .initialization_script(WINDOW_CONTROLS)
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

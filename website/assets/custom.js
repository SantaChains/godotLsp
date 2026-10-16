// 精简版页面增强（对齐 rust-course 视觉，mdbook 0.5 兼容）：
// 1) 右侧页内目录（pagetoc）  2) 图片点击放大（BigPicture）
// 去掉了 rust-course 的 giscus 评论与主题联动段（本站不使用评论系统）。

(function () {
    "use strict";

    function init() {
        var path = window.location.pathname;
        if (path.endsWith("/print.html")) return;

        // 图片点击放大
        Array.prototype.forEach.call(document.querySelectorAll("main img"), function (img) {
            img.addEventListener("click", function () {
                if (typeof BigPicture !== "undefined") {
                    BigPicture({ el: img });
                }
            });
        });

        // 右侧页内目录：main 下的 h1/h2（mdBook 渲染的标题自带 id）
        var pagetoc = document.getElementsByClassName("pagetoc")[0];
        if (!pagetoc) return;

        var headings = document.querySelectorAll("main h1, main h2");
        if (headings.length === 0) return;

        pagetoc.innerHTML = "";
        Array.prototype.forEach.call(headings, function (el) {
            var link = document.createElement("a");
            link.href = "#" + el.id;
            link.appendChild(document.createTextNode(el.textContent));
            link.style.paddingLeft = el.tagName === "H1" ? "10px" : "20px";
            pagetoc.appendChild(link);
        });

        var links = pagetoc.getElementsByTagName("a");

        function update() {
            var current = null;
            Array.prototype.forEach.call(headings, function (el, i) {
                if (el.getBoundingClientRect().top <= 120 && links[i]) {
                    current = links[i];
                }
            });
            Array.prototype.forEach.call(links, function (el) {
                el.classList.remove("active");
            });
            if (current) current.classList.add("active");
        }

        window.addEventListener("scroll", update);
        update();
    }

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", init);
    } else {
        init();
    }
})();

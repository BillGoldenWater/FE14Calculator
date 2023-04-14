/*
 *  Copyright 2023 Golden_Water, 火纹梅戚
 *  SPDX-License-Identifier: AGPL-3.0-only
 */

var cacheName = "f14calc-pwa";
var filesToCache = [
  "./",
  "./index.html",
  "./styles.css",
  "./fe14_calculator_ui_bg.wasm",
  "./fe14_calculator_ui.js",
  "./public/avatar/阿尔弗雷德.webp",
  "./public/avatar/艾比.webp",
  "./public/avatar/艾媞耶.webp",
  "./public/avatar/爱尔.webp",
  "./public/avatar/安巴.webp",
  "./public/avatar/安娜.webp",
  "./public/avatar/奥尔坦希亚.webp",
  "./public/avatar/贝珥.webp",
  "./public/avatar/波聂.webp",
  "./public/avatar/布修隆.webp",
  "./public/avatar/帝亚曼德.webp",
  "./public/avatar/凡德雷.webp",
  "./public/avatar/佛贾特.webp",
  "./public/avatar/芙岚.webp",
  "./public/avatar/戈尔德玛莉.webp",
  "./public/avatar/孤雷葛里.webp",
  "./public/avatar/花月.webp",
  "./public/avatar/贾恩.webp",
  "./public/avatar/杰尔科巴.webp",
  "./public/avatar/洁德.webp",
  "./public/avatar/柯岚.webp",
  "./public/avatar/克萝艾.webp",
  "./public/avatar/拉法尔.webp",
  "./public/avatar/菈琵思.webp",
  "./public/avatar/霖丹.webp",
  "./public/avatar/琉尔.webp",
  "./public/avatar/路易.webp",
  "./public/avatar/罗萨德.webp",
  "./public/avatar/玛德琳.webp",
  "./public/avatar/玫琳.webp",
  "./public/avatar/蜜丝提拉.webp",
  "./public/avatar/莫布.webp",
  "./public/avatar/帕涅托涅.webp",
  "./public/avatar/庞德罗.webp",
  "./public/avatar/赛安达斯.webp",
  "./public/avatar/赛勒斯提雅.webp",
  "./public/avatar/史塔卢克.webp",
  "./public/avatar/希特丽妮卡.webp",
  "./public/avatar/锡莉奴.webp",
  "./public/avatar/尤娜卡.webp",
  "./public/avatar/札菲亚.webp",
];

/* Start the service worker and cache all of the app's content */
self.addEventListener("install", function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

/* Serve cached content when offline */
self.addEventListener("fetch", function (e) {
  e.respondWith(
    caches.match(e.request).then(function (response) {
      if (response) {
        fetch(e.request)
          .then(async function (res) {
            const cache = await caches.open(cacheName);
            await cache.delete(e.request);
            await cache.put(e.request, res);
          })
          .catch(() => undefined);
        return response;
      } else {
        return fetch(e.request);
      }
    })
  );
});

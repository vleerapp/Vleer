<template>
  <div id="player" class="player">
    <div class="song-info">
      <div style="--bgsrc: url('/empty.png')" class="song-img empty" border="none" id="img">
      </div>
      <div class="text-info">
        <div class="name empty" id="name"></div>
        <div class="artist empty" id="artist"></div>
      </div>
    </div>
    <div class="controlls">
      <div class="progress-controlls">
        <img src="/svg/bold/backward.svg" id="back" class="back" />
        <img src="/svg/bold/play.svg" id="pauseplay" class="pause-play" />
        <img src="/svg/bold/forward.svg" id="for" class="for" />
      </div>
      <div class="progressbar">
        <div class="progress-time" id="progress-time">0:00</div>
        <div class="bar">
          <div class="bar-bg"></div>
          <div style="width: 0%" class="bar-filled" id="progressbar"></div>
        </div>
        <div class="time-left" id="time-left">0:00</div>
      </div>
    </div>
    <div class="sound-controlls">
      <img src="/svg/bold/volume-mute.svg" class="less" />
      <div class="bar">
        <div class="bar-bg"></div>
        <div style="width: 100%" class="bar-filled"></div>
      </div>
      <img src="/svg/bold/volume-high.svg" class="more" />
    </div>
  </div>
</template>

<script>
import { MusicHandler } from '/musicHandler'

const musicHandler = MusicHandler.getInstance();

musicHandler.onPlayEvent(async () => {
  var info = await musicHandler.getInfo()
});

function playpause() {
  let imgsrc = document.getElementById("pauseplay");
  imgsrc.classList.add("clickAnimation");
  window.setTimeout(function () {
    if (musicHandler.audio.paused) {
      imgsrc.src = "/svg/bold/pause.svg";
      musicHandler.play();
    } else {
      imgsrc.src = "/svg/bold/play.svg";
      musicHandler.pause();
    }
  }, 150);
  window.setTimeout(function () {
    imgsrc.classList.remove("clickAnimation");
  }, 400);
}

function back() {
  var source = document.getElementById("media");
  let imgsrc = document.getElementById("back");
  imgsrc.classList.add("clickAnimation");
  source.currentTime = 0;
  window.setTimeout(function () {
    imgsrc.classList.remove("clickAnimation");
  }, 400);
}

export default {
  async mounted() {
    document
      .getElementById("pauseplay")
      .addEventListener("click", () => {
        musicHandler.pauseplay();
      });

    document
      .getElementById("back")
      .addEventListener("click", () => {
        back();
      });

    var progressBar = document.querySelector(".progressbar > .bar");
    var progressBarFill = document.querySelector(
      ".progressbar > .bar >.bar-filled"
    );

    progressBar.addEventListener("mousedown", function (event) {
      musicHandler.isDragging = true;
      musicHandler.dragX = event.offsetX;
    });

    progressBar.addEventListener("mousemove", function (event) {
      if (musicHandler.isDragging) {
        var progress =
          ((event.clientX - progressBar.getBoundingClientRect().left) /
            progressBar.clientWidth) *
          100;
        if (progress > 100) progress = 100
        progressBarFill.style.width = progress + "%";
        musicHandler.dragX = event.clientX - progressBar.getBoundingClientRect().left;
      }
    });

    progressBar.addEventListener("mouseup", function (event) {
      if (musicHandler.isDragging) {
        musicHandler.isDragging = false;
        var progress =
          ((event.clientX - progressBar.getBoundingClientRect().left) /
            progressBar.clientWidth) *
          100;
        progressBarFill.style.width = progress + "%";
        musicHandler.audio.currentTime = (musicHandler.dragX / progressBar.clientWidth) * musicHandler.audio.duration;
      }
    });

    progressBar.addEventListener("mouseleave", function (event) {
      if (musicHandler.isDragging) {
        musicHandler.isDragging = false;
        var progress =
          ((event.clientX - progressBar.getBoundingClientRect().left) /
            progressBar.clientWidth) *
          100;
        progressBarFill.style.width = progress + "%";
        musicHandler.audio.currentTime = (musicHandler.dragX / progressBar.clientWidth) * musicHandler.audio.duration;
      }
    });

    var AudioBar = document.querySelector(".sound-controlls > .bar");
    var AudioBarFill = document.querySelector(
      ".sound-controlls > .bar > .bar-filled"
    );

    var isDraggingAudio = false;
    var dragAudioX = 0;

    AudioBar.addEventListener("mousedown", function (event) {
      isDraggingAudio = true;
      dragAudioX = event.offsetX;
    });

    AudioBar.addEventListener("mousemove", function (event) {
      if (isDraggingAudio) {
        var progress =
          ((event.clientX - AudioBar.getBoundingClientRect().left) /
            AudioBar.clientWidth) *
          100;
        if (progress > 100) progress = 100
        AudioBarFill.style.width = Math.floor(progress) + "%";
        musicHandler.volume((dragAudioX / AudioBar.clientWidth) * 0.3);
        dragAudioX = event.clientX - AudioBar.getBoundingClientRect().left;
      }
    });

    AudioBar.addEventListener("mouseup", function (event) {
      if (isDraggingAudio) {
        isDraggingAudio = false;
        var progress =
          ((event.clientX - AudioBar.getBoundingClientRect().left) /
            AudioBar.clientWidth) *
          100;
        if (progress > 100) progress = 100
        AudioBarFill.style.width = Math.floor(progress) + "%";
        musicHandler.volume((dragAudioX / AudioBar.clientWidth) * 0.3);
        audio.setAttribute("svolume", (dragAudioX / AudioBar.clientWidth) * 0.3)
      }
    });

    AudioBar.addEventListener("mouseleave", function (event) {
      if (isDraggingAudio) {
        isDraggingAudio = false;
        var progress =
          ((event.clientX - AudioBar.getBoundingClientRect().left) /
            AudioBar.clientWidth) *
          100;
        if (progress > 100) progress = 100
        AudioBarFill.style.width = Math.floor(progress) + "%";
        musicHandler.volume((dragAudioX / AudioBar.clientWidth) * 0.3);
        audio.setAttribute("svolume", (dragAudioX / AudioBar.clientWidth) * 0.3)
      }
    });
  },
};
</script>

<style>
@import "~/css/player.css";
@import "~/css/media.css";
</style>

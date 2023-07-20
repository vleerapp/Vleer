<template>
  <NuxtLayout name="page">
    <h1 class="page-title">Settings</h1>
    <div class="page">
      <input v-model="userName" type="text" id="input" class="userName" placeholder="Username" />
      <button @click="selectAvatar" id="saveButton" class="avatar"><img class="avatarSvg" id="img"
          src="/svg/linear/profile-circle.svg">Avatar</button>

      <button @click="saveFile" class="save">Save</button>
    </div>
  </NuxtLayout>
</template>

<script setup lang="ts">
import { writeTextFile, BaseDirectory, createDir, exists, readTextFile, copyFile } from "@tauri-apps/api/fs";
import { ref } from "vue";
import { open } from '@tauri-apps/api/dialog';

interface Contents {
  userName: String;
  avatarPath: String;
}

if (!(await exists("", { dir: BaseDirectory.AppConfig }))) {
  await createDir("", {
    dir: BaseDirectory.AppConfig,
    recursive: true,
  });
}

if (!(await exists("config.json", { dir: BaseDirectory.AppConfig }))) {
  await writeTextFile("config.json", "{}", {
    dir: BaseDirectory.AppConfig,
  });
}

const userName = ref("");

var contents = await readTextFile("config.json", {
  dir: BaseDirectory.AppConfig,
});

var avatarPath: String;

async function selectAvatar() {
  const selectedAvatar = await open({
    multiple: false,
    filters: [{
      name: "Image",
      extensions: ["png", "jpeg", "jpeg"]
    }],
  });

  if (typeof selectedAvatar === "string") await copyFile(selectedAvatar, "avatar.png", {
    dir: BaseDirectory.AppConfig,
  });

  avatarPath = String(selectedAvatar);
  saveFile();
}

async function saveFile() {
  try {
    if (!(await exists("", { dir: BaseDirectory.AppConfig }))) {
      await createDir("", {
        dir: BaseDirectory.AppConfig,
        recursive: true,
      });
    }

    var parsedContents = JSON.parse(contents) as Contents;

    parsedContents["userName"] = userName.value;
    parsedContents["avatarPath"] = String(avatarPath);


    await writeTextFile("config.json", JSON.stringify(parsedContents), {
      dir: BaseDirectory.AppConfig
    });

    var saveButton = document.getElementById("saveButton");
    if (saveButton) {
      saveButton.classList.add('saved');
    }

  } catch (err) {
    console.log(err);
  }
}
</script>

<style>
@import '../css/style.css';
@import '../css/settings.css';
</style>
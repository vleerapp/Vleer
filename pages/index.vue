<template>
  <div>
    <input v-model="videoUrl" type="text" placeholder="Enter YouTube URL" />
    <button @click="downloadAudio">Download</button>
  </div>
</template>

<script>
export default {
  data() {
    return {
      videoUrl: ""
    }
  },
  methods: {
    downloadAudio() {
      window.location.href = `/api/download?url=${encodeURIComponent(this.videoUrl)}`;
    },
  },
};
</script>

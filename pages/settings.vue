<template>
  <div class="main element">
    <p class="element-title">Settings</p>
    <div class="settings">
      <p>Equalizer</p>
      <div class="equalizer">
        <div class="info">
          <p>+12.0</p>
          <p>0.0</p>
          <p>-12.0</p>
        </div>
        <div class="sliders">
          <div v-for="(freq, index) in frequencies" :key="freq" class="freq">
            <input type="number" v-model.number="eqGains[index]" @input="updateEqGain(index, $event.target.valueAsNumber)" class="gain" step="0.1" min="-12" max="12">
            <input type="range" min="-12" max="12" step="0.1" v-model.number="eqGains[index]"
              @input="updateEqGain(index, eqGains[index])" class="range">
            <div class="hz">{{ formatFrequency(freq) }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { type EQSettings } from '~/types/types';

const { $music, $settings } = useNuxtApp()

const frequencies = [32, 64, 125, 250, 500, 1000, 2000, 4000, 8000, 16000];
const eqGains = ref(new Array(frequencies.length).fill(0.0));

const eq = $settings.getEq()
frequencies.forEach((freq, index) => {
  const freqKey = freq.toString();
  eqGains.value[index] = Number(parseFloat(eq[freqKey as keyof EQSettings] || 0).toFixed(1));
});

function updateEqGain(filterIndex: number, gain: number) {
  if (isNaN(gain)) return;
  let formattedGain = Math.min(12, Math.max(gain, -12));
  formattedGain = Number(formattedGain.toFixed(1));
  eqGains.value[filterIndex] = formattedGain;
  $music.setEqGain(filterIndex, formattedGain);
  const eqSettingsMap = $settings.getEq();
  eqSettingsMap[frequencies[filterIndex].toString() as keyof EQSettings] = formattedGain;
  $settings.setEq(eqSettingsMap as EQSettings);
}

function formatFrequency(freq: number) {
  return freq >= 1000 ? `${freq / 1000}KHz` : `${freq}Hz`;
}

function resetEQ() {
  frequencies.forEach((freq, index) => {
    eqGains.value[index] = 0;
  });
  const eqSettingsMap = {} as EQSettings;
  frequencies.forEach((freq, index) => {
    $music.setEqGain(index, 0);
    eqSettingsMap[freq.toString() as keyof EQSettings] = "0";
  });
  $settings.setEq(eqSettingsMap as EQSettings)
}
</script>

<style scoped lang="scss">
@import '~/assets/styles/pages/settings.scss';
</style>
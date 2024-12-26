<template>
  <div class="main element">
    <p class="element-title">Settings</p>
    <div class="settings">
      <div class="equalizer setting">
        <p>Equalizer</p>
        <div class="eq">
          <div class="info">
            <p>+12.0</p>
            <p>0.0</p>
            <p>-12.0</p>
          </div>
          <div class="sliders">
            <div v-for="(freq, index) in frequencies" :key="freq" class="freq">
              <input
                :max="12"
                :min="-12"
                :step="0.1"
                @input="updateEqGain(index, ($event.target as HTMLInputElement)?.valueAsNumber ?? 0)"
                class="gain"
                type="number"
                v-model.number="eqGains[index]"
              >
              <input
                :max="12"
                :min="-12"
                :step="0.1"
                @input="updateEqGain(index, eqGains[index])"
                class="range"
                type="range"
                v-model.number="eqGains[index]"
              >
              <div class="hz">{{ formatFrequency(freq) }}</div>
            </div>
          </div>
        </div>
      </div>

      <div class="api-url setting">
        <p>Search API URL</p>
        <div class="input-container">
          <input
            @input="updateApiURL"
            class="input"
            placeholder="url"
            spellcheck="false"
            type="url"
            v-model="apiUrl"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue';
import type { Settings } from '~/types/types';
import { emit } from '@tauri-apps/api/event';

const { $settings } = useNuxtApp();

const apiUrl = ref('');
const eqGains = ref<number[]>([]);
const frequencies = [32, 64, 125, 250, 500, 1000, 2000, 4000, 8000, 16000];

onMounted(async () => {
  apiUrl.value = await $settings.getApiUrl();
  const eq = await $settings.getEq();
  eqGains.value = frequencies.map(freq => Number(parseFloat(eq.values[freq.toString()] || '0').toFixed(1)));
});

async function updateApiURL() {
  await $settings.setApiUrl(apiUrl.value);
}

async function updateEqGain(filterIndex: number, gain: number) {
  if (isNaN(gain)) return;
  const formattedGain = Number(Math.min(12, Math.max(gain, -12)).toFixed(1));
  eqGains.value[filterIndex] = formattedGain;
  
  const eqSettingsMap = await $settings.getEq();
  eqSettingsMap.values[frequencies[filterIndex].toString()] = formattedGain.toString();
  await $settings.setEq(eqSettingsMap);
  
  await emit('eq-change', eqSettingsMap);
}

function formatFrequency(freq: number): string {
  return freq >= 1000 ? `${freq / 1000}KHz` : `${freq}Hz`;
}

async function resetEQ() {
  const resetEqGains = new Array(frequencies.length).fill(0);
  eqGains.value = resetEqGains;
  
  const eqSettingsMap: Settings['eq'] = { values: {} };
  frequencies.forEach(freq => {
    eqSettingsMap.values[freq.toString()] = '0';
  });
  await $settings.setEq(eqSettingsMap);
  
  await emit('eq-change', eqSettingsMap);
}
</script>

<style scoped lang="scss">
@use '~/assets/styles/pages/settings.scss';
</style>
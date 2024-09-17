<template>
  <div class="navbar element">
    <p class="element-title">Navbar</p>
    <div class="links">
      <NuxtLink class="link" to="/">
        <div class="svg-active">
          <IconsNavHomeFill />
        </div>
        <div class="svg-default">
          <IconsNavHomeOutline />
        </div>
        Home
      </NuxtLink>
      <div class="link search-link" @click="toggleSearch">
        <template v-if="!isSearchActive">
          <div class="svg-active">
            <IconsNavSearchFill />
          </div>
          <div class="svg-default">
            <IconsNavSearchOutline />
          </div>
          Search
        </template>

        <div v-else class="search-container">
          <IconsNavSearchOutline class="icon"/>
          <input 
            ref="searchInput" 
            v-model="searchTerm" 
            class="input" 
            type="text" 
            placeholder="Search"
            @blur="handleBlur" 
            @focus="handleFocus" 
            @input="handleInput"
            @keydown="handleKeyDown"
          />
        </div>
      </div>
    </div>

    <div>
      <NuxtLink class="link" to="/settings">
        <div class="svg-active">
          <IconsNavSettingsFill />
        </div>
        <div class="svg-default">
          <IconsNavSettingsOutline />
        </div>
      </NuxtLink>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { debounce } from 'lodash-es';
import { ref } from 'vue';
import { useRouter } from 'vue-router';

const isSearchActive = ref(false);
const router = useRouter();
const searchInput = ref<HTMLInputElement | null>(null);
const searchTerm = ref('');

const handleBlur = () => {
  if (searchTerm.value === '') {
    isSearchActive.value = false;
  }
};

const handleFocus = () => {
  if (searchTerm.value !== '') {
    router.push({ path: '/search', query: { q: searchTerm.value } });
  }
};

const handleInput = debounce(() => {
  if (searchTerm.value.trim() !== '') {
    router.push({ path: '/search', query: { q: searchTerm.value.trim() } });
  }
}, 300);

const handleKeyDown = (event: KeyboardEvent) => {
  if (event.key === 'Enter') {
    event.preventDefault();
    if (searchTerm.value.trim() !== '') {
      router.push({ path: '/search', query: { q: searchTerm.value.trim() } });
    }
  }
};

const toggleSearch = () => {
  if (!isSearchActive.value) {
    isSearchActive.value = true;
    router.push('/search');
    setTimeout(() => {
      searchInput.value?.focus();
    }, 0);
  } else if (isSearchActive.value && searchTerm.value === '') {
    isSearchActive.value = false;
  }
};
</script>

<style scoped lang="scss">
@import "~/assets/styles/components/navbar.scss";
</style>

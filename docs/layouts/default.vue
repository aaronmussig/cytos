<script lang="ts" setup>
// Extract the version from an environment variable
import NavigationBar from "~/components/layout/NavigationBar.vue";

const runtimeConfig = useRuntimeConfig();
const version = runtimeConfig.public.VERSION || '';
</script>

<template>
  <div class="min-h-screen">
    <div class="container mx-auto h-full text-cyan-50 pb-8 pt-3 px-3 lg:px-20">

      <!-- Header -->
      <div class="text-xl font-bold text-center">
        <template v-if="$route.path === '/'">
          cytos v{{ version }}
        </template>
        <template v-else>
          <nuxt-link to="/">cytos v{{ version }}</nuxt-link>
        </template>
      </div>

      <!-- Navigation menu -->
      <div class="mt-3">
        <NavigationBar/>
      </div>

      <!-- Content -->
      <div class="mt-5">
        <slot/>
      </div>

    </div>
  </div>

</template>

<style>
.page-enter-active,
.page-leave-active {
  transition: all 0.2s;
}

.page-enter-from,
.page-leave-to {
  opacity: 0;
}


html {
  scrollbar-gutter: stable both-edges;
  background-color: #111827; /* bg-gray-900 */
}
</style>

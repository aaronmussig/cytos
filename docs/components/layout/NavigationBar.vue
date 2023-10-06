<script lang="ts" setup>
// Split the route path into an array of strings
function splitRoute(path: string): [string, string][] {
  const parts = path.split('/').filter(part => part !== ''); // split the path and remove any empty strings
  const paths: [string, string][] = [["/", "cytos"]];

  let currentPath = '';
  for (let part of parts) {
    currentPath += '/' + part; // build the path step by step
    paths.push([currentPath, part]);
  }

  return paths;
}
</script>

<template>
  <div class="flex justify-center">
    <div class="flex bg-cyan-950 border-2 border-cyan-900 py-1 px-3 rounded-lg font-mono text-sm">
      <div class="my-auto">
        &gt;
      </div>
      <div
          v-for="(route, index) of splitRoute($route.path)"
          class="p-1 mx-1"
      >
        <template v-if="index === splitRoute($route.path).length - 1">
          {{ route[1] }}
        </template>
        <template v-else>
          <nuxt-link :to="route[0]">
            {{ route[1] }}
          </nuxt-link>
        </template>
      </div>
    </div>

  </div>
</template>

<style scoped>

</style>

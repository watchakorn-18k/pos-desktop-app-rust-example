<script setup lang="ts">
import ProductList from "../components/ProductList.vue";
</script>

<template>
    <main class="flex flex-col gap-5">
        <ProductList />
        <router-link
            to="/food"
            class="bg-blue-500 hover:bg-blue-400 w-full rounded-lg p-4 m-2 text-xl text-white text-center"
        >
            สั่งอาหาร
        </router-link>
    </main>
</template>

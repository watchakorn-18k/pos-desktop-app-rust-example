<script setup lang="ts">
import NavBar from "./components/Navbar.vue";
import Footer from "./components/Footer.vue";
</script>

<template>
    <NavBar />
    <div class="flex flex-col justify-between h-screen">
        <div class="container mx-auto my-auto p-4">
            <router-view />
        </div>

        <Footer />
    </div>
</template>

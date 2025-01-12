<script setup lang="ts">
import DialogAlert from "../components/DialogAlert.vue";
import { ref, onMounted } from "vue";
import CardProduct from "./CardProduct.vue";
import { useProductStore } from "../stores/productStore";
import type { Product } from "../models/product";

const productStore = useProductStore();
const foods = ref([] as Product[]);
const isModalAddProductOpen = ref(false);

onMounted(async () => {
    await productStore.loadProducts();
    foods.value = productStore.products;
});

const removeProduct = (id: string) => {
    productStore.removeProduct(id);
    foods.value = productStore.products;
};

const addProduct = () => {
    isModalAddProductOpen.value = true;
};

const addProductToStore = async (product: Product) => {
    await productStore.addProduct(product);
    foods.value = productStore.products;
};
</script>

<template>
    <DialogAlert
        :isModalOpen="isModalAddProductOpen"
        @close="isModalAddProductOpen = false"
        @add-product="addProductToStore"
    />
    <div class="flex flex-col gap-4">
        <div class="flex justify-between">
            <h1 class="text-3xl text-center">Products</h1>
            <button
                class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded"
                @click="addProduct"
            >
                เพิ่มสินค้า
            </button>
        </div>
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
            <CardProduct
                v-for="food in foods"
                :key="food.id"
                :id="food.id"
                :name="food.name"
                :price="food.price"
                :description="food.description"
                @remove="removeProduct"
            />
        </div>
    </div>
</template>

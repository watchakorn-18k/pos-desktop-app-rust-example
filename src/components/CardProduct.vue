<script setup lang="ts">
import { defineProps, defineEmits } from "vue";
import type { Product } from "../models/product";
const props = defineProps<{
    id: string;
    name: string;
    price: number;
    description: string;
}>();

const emits = defineEmits<{
    (e: "remove", product: string): void;
    (e: "update", product: Product): void;
}>();

const remove = () => {
    emits("remove", props.id);
};
</script>

<template>
    <div class="card border-1 border-gray-400 shadow-md p-4 m-2 rounded-lg">
        <div class="card-body flex flex-col gap-5">
            <div class="flex justify-between items-center">
                <h2 class="card-title text-xl px-4">
                    {{ props.name }} (฿{{ props.price }})
                </h2>
                <button
                    class="bg-red-500 text-white px-4 py-2 rounded-lg hover:bg-red-600"
                    @click="remove"
                >
                    X
                </button>
            </div>
            <p class="card-text text-sm px-4">
                {{ props.description }}
            </p>
        </div>
    </div>
</template>

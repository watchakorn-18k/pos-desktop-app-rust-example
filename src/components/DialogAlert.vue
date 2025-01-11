<script setup lang="ts">
import { reactive, defineEmits, ref, defineProps, watch } from "vue";
import { v4 as uuidv4 } from "uuid";
import type { Product } from "../models/product";

const product = reactive<Product>({
    id: "",
    name: "",
    price: 0,
    description: "",
});

const emit = defineEmits(["add-product", "close"]);
const props = defineProps<{
    isModalOpen: boolean;
}>();

const dialogRef = ref<HTMLDialogElement | null>(null);

watch(
    () => props.isModalOpen,
    (newValue) => {
        if (newValue) {
            dialogRef.value?.showModal(); // เปิด Modal
        } else {
            dialogRef.value?.close(); // ปิด Modal
        }
    },
);

const clearProduct = () => {
    product.id = "";
    product.name = "";
    product.price = 0;
    product.description = "";
};

const addProduct = () => {
    product.id = uuidv4();
    const productSend = Object.assign({}, product); // คัดลอก object
    emit("add-product", productSend);
    emit("close"); // ปิด Modal หลังจากเพิ่มสินค้า
    clearProduct();
};

const closeModal = () => {
    emit("close"); // ส่ง event ปิด Modal
};
</script>

<template>
    <dialog
        ref="dialogRef"
        class="relative z-10"
        aria-labelledby="modal-title"
        role="dialog"
        aria-modal="true"
    >
        <!-- ส่วนของ Modal -->
        <div
            class="fixed inset-0 bg-gray-500/75 transition-opacity"
            aria-hidden="true"
        ></div>
        <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
            <div
                class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0"
            >
                <div
                    class="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg"
                >
                    <!-- ส่วนเนื้อหาของ Modal -->
                    <div class="bg-white px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
                        <h3
                            class="text-xl font-semibold text-gray-900"
                            id="modal-title"
                        >
                            Add Product
                        </h3>
                        <div class="mt-2">
                            <div class="mt-2">
                                <label
                                    for="name"
                                    class="block text-sm font-medium text-gray-700"
                                >
                                    Name
                                </label>
                                <div class="mt-1">
                                    <input
                                        v-model="product.name"
                                        type="text"
                                        name="name"
                                        id="name"
                                        class="block w-full rounded-md border-gray-200 border-2"
                                    />
                                </div>
                            </div>
                            <div class="mt-2">
                                <label
                                    for="price"
                                    class="block text-sm font-medium text-gray-700"
                                >
                                    Price
                                </label>
                                <div class="mt-1">
                                    <input
                                        v-model="product.price"
                                        type="number"
                                        name="price"
                                        id="price"
                                        class="block w-full rounded-md border-gray-200 border-2"
                                    />
                                </div>
                            </div>
                            <div class="mt-2">
                                <label
                                    for="description"
                                    class="block text-sm font-medium text-gray-700"
                                >
                                    Description
                                </label>
                                <div class="mt-1">
                                    <textarea
                                        id="description"
                                        name="description"
                                        v-model="product.description"
                                        rows="3"
                                        class="block w-full rounded-md border-gray-200 border-2"
                                        placeholder="Add a description"
                                    ></textarea>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div
                        class="bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6"
                    >
                        <button
                            type="button"
                            class="inline-flex w-full justify-center rounded-md bg-green-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-green-500 sm:ml-3 sm:w-auto"
                            @click="addProduct"
                        >
                            Add
                        </button>
                        <button
                            type="button"
                            class="mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:mt-0 sm:w-auto"
                            @click="closeModal"
                        >
                            Cancel
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </dialog>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useProductStore } from "../stores/productStore";
import type { Product } from "../models/product";
import DialogPayment from "../components/DialogPayment.vue";

const products = ref<Product[]>([]);
const isPaymentSuccess = ref(false);
const productStore = useProductStore();

interface CartItem {
    name: string;
    price: number;
    quantity: number;
}

const cart = ref<CartItem[]>([]);

// ฟังก์ชันเพิ่มสินค้าในตะกร้า
const addToCart = (product: Product) => {
    const found = cart.value.find((item) => item.name === product.name);
    if (found) {
        found.quantity++;
    } else {
        cart.value.push({ ...product, quantity: 1 });
    }
};

// ฟังก์ชันลบสินค้าจากตะกร้า
const removeFromCart = (item: CartItem) => {
    cart.value = cart.value.filter((cartItem) => cartItem !== item);
};

// คำนวณยอดรวม
const totalAmount = computed(() => {
    return cart.value.reduce(
        (total, item) => total + item.price * item.quantity,
        0,
    );
});

// ฟังก์ชันการจ่ายเงิน
const checkout = () => {
    isPaymentSuccess.value = true;
    cart.value = [];
};

// โหลดสินค้าเมื่อ component ถูกโหลด
onMounted(async () => {
    await productStore.loadProducts();
    products.value = productStore.products;
});
</script>

<template>
    <!-- DialogPayment จะแสดงเฉพาะเมื่อ isPaymentSuccess เป็น true -->
    <DialogPayment
        :isPaymentSuccess="isPaymentSuccess"
        @close="isPaymentSuccess = false"
    />

    <div class="bg-gray-100 min-h-screen flex flex-col justify-between">
        <main class="flex-grow p-6">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <!-- รายการสินค้า -->
                <div class="bg-white p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-semibold mb-4">สินค้า</h2>
                    <ul>
                        <li
                            v-for="(product, index) in products"
                            :key="index"
                            class="flex justify-between items-center py-2 border-b"
                        >
                            <span>{{ product.name }}</span>
                            <span>฿{{ product.price.toFixed(2) }}</span>
                            <button
                                class="bg-blue-500 text-white px-3 py-1 rounded"
                                @click="addToCart(product)"
                            >
                                เพิ่ม
                            </button>
                        </li>
                    </ul>
                </div>

                <!-- ตะกร้าสินค้า -->
                <div class="bg-white p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-semibold mb-4">ตะกร้าสินค้า</h2>
                    <ul>
                        <li
                            v-for="(item, index) in cart"
                            :key="index"
                            class="flex justify-between items-center py-2 border-b"
                        >
                            <span>{{ item.name }} x{{ item.quantity }}</span>
                            <span
                                >฿{{
                                    (item.price * item.quantity).toFixed(2)
                                }}</span
                            >
                            <button
                                class="bg-red-500 text-white px-3 py-1 rounded"
                                @click="removeFromCart(item)"
                            >
                                ลบ
                            </button>
                        </li>
                    </ul>
                    <div class="mt-4 flex justify-between items-center">
                        <span class="font-bold">ยอดรวม:</span>
                        <span>฿{{ totalAmount.toFixed(2) }}</span>
                    </div>
                    <button
                        class="mt-6 bg-green-500 text-white py-2 px-4 w-full rounded"
                        @click="checkout"
                        :disabled="cart.length === 0"
                        :class="{ 'opacity-50': cart.length === 0 }"
                    >
                        จ่ายเงิน
                    </button>
                </div>
            </div>
        </main>
    </div>
</template>

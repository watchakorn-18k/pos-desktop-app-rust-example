import { defineStore } from "pinia";
import type { Product } from "../models/product";
import { FoodMocks } from "../mock/products";
import { invoke } from "@tauri-apps/api/core";

export const useProductStore = defineStore("product", {
  state: () => ({
    products: [] as Product[],
    product: {} as Product,
  }),
  actions: {
    async loadProducts() {
      let data: any;
      try {
        data = await invoke("get_products");
      } catch (error) {
        data = FoodMocks;
      }
      const products = data.map((product: any) => {
        return {
          id: product.id,
          name: product.name,
          price: product.price,
          description: product.description,
        };
      });
      this.products = products;
    },
    setProducts(products: Product[]) {
      this.products = products;
    },
    setProduct(product: Product) {
      this.product = product;
    },
    async removeProduct(productId: string) {
      this.products = this.products.filter(
        (product) => product.id !== productId,
      );
      await invoke("remove_product", { id: productId });
    },
    async addProduct(product: Product) {
      await invoke("add_product", {
        name: product.name,
        price: product.price,
        description: product.description,
      });
      this.products.push(product);
    },
    async updateProduct(product: Product) {
      await invoke("update_product", {
        id: product.id,
        name: product.name,
        price: product.price,
        description: product.description,
      });
      this.products = this.products.map((p) => {
        if (p.id === product.id) {
          return product;
        }
        return p;
      });
    },
  },
});

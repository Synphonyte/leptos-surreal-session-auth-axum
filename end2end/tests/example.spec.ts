import { test, expect } from "@playwright/test";

test("homepage has title and links to intro page", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await expect(page).toHaveTitle("Welcome to Leptos Surrealdb Axum Session Auth");

  await expect(page.locator("h1")).toHaveText("hola");
});

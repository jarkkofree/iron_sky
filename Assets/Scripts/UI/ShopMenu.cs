using System;
using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;
using UnityEngine.AddressableAssets;
using UnityEngine.ResourceManagement.AsyncOperations;
using UnityEngine.UI;

namespace ShopUI.UI
{
    public class ShopMenu : MonoBehaviour
    {
        [SerializeField] private Transform _shopGrid;
        [SerializeField] private AssetReference _shopGridItem;
        [SerializeField] private TextMeshProUGUI _shopName;

        private Shop _shop;

        private void Awake()
        {
            GameShop.OnShopSelected += ShopSelected;
        }

        private void OnDestroy()
        {
            GameShop.OnShopSelected -= ShopSelected;
        }

        private void ShopSelected(Shop shop)
        {
            _shop = shop;

            if (_shop == null) return;
            
            Show(_shop.Inventory.RandomizeInventory());
        }

        // Get shop name from data (scriptable object)
        // Populate grid item text with Item Name (from currentInventory)
        //      add Price
        private async void Show(List<ShopItem> currentInventory)
        {
            foreach (ShopItem item in currentInventory)
            {
                var gridItem = await _shopGridItem.InstantiateAsyncGetComponent<TextMeshProUGUI>();
                gridItem.transform.SetParent(_shopGrid, false);
                gridItem.transform.localScale = Vector3.one;
                gridItem.text = $"{item.DisplayName}, Cr. {item.BasePrice}";
            }
            _shopName.text = _shop.DisplayName;
        }
    }
}

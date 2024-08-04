using System;
using System.Collections.Generic;
using TMPro;
using UnityEngine;
using UnityEngine.AddressableAssets;
using UnityEngine.ResourceManagement.AsyncOperations;

namespace ShopUI.UI
{
    public class ShopMenu : MonoBehaviour
    {
        [SerializeField] private Transform _shopGrid;
        [SerializeField] private AssetReference _shopGridItem;
        [SerializeField] private TextMeshProUGUI _shopName;
        [SerializeField] private TextMeshProUGUI _creditAccount;
        [SerializeField] private PlayerData _player;

        private void Awake()
        {
            ShowShopButton.OnClicked += ShowShopButtonClicked;
            PlayerInventory.OnCreditsChanged += PlayerCreditsChanged;
        }

        private void OnDestroy()
        {
            ShowShopButton.OnClicked -= ShowShopButtonClicked;
            PlayerInventory.OnCreditsChanged -= PlayerCreditsChanged;
        }

        private void PlayerCreditsChanged(int credits)
        {
            _creditAccount.text = $"Cr. {credits}";
        }

        private void ShowShopButtonClicked(ShopData shop)
        {
            Show(shop);
        }

        private async void Show(ShopData shop)
        {
            var items = shop.Inventory.RandomizeInventory();
            foreach (ShopItem item in items)
            {
                var gridItem = await _shopGridItem.InstantiateAsyncGetComponent<TextMeshProUGUI>();
                gridItem.transform.SetParent(_shopGrid, false);
                gridItem.transform.localScale = Vector3.one;
                gridItem.text = $"{item.DisplayName}, Cr. {item.BasePrice}";
            }
            _shopName.text = shop.DisplayName;
        }
    }
}

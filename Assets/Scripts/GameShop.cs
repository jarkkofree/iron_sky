using ShopUI.UI;
using System;
using UnityEngine;

public class GameShop : MonoBehaviour
{
    [SerializeField] private Shop _shop;

    public static Action<Shop> OnShopSelected;

    private void Start()
    {
        OnShopSelected?.Invoke(_shop);
    }
}
using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

public class ShowShopButton : MonoBehaviour
{
    private Button _button;
    [SerializeField] private ShopData _shop;

    public static Action<ShopData> OnClicked;

    private void Awake()
    {
        _button = GetComponent<Button>();
        _button.onClick.AddListener(()=>
        {
            _button.interactable = false;
            OnClicked?.Invoke(_shop);
        });
    }

    private void OnDestroy()
    {
        _button.onClick.RemoveAllListeners();
    }
}

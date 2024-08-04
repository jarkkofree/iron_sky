using System;
using UnityEngine;
using UnityEngine.UI;

public class AddCreditsButton : MonoBehaviour
{
    [SerializeField] private int _creditsToAdd;
    private Button _button;

    private void Awake()
    {
        _button = GetComponent<Button>();
        _button.onClick.AddListener(() =>
        {
            Player.Inventory.AddCredits(_creditsToAdd);
        });
    }

    private void OnDestroy()
    {
        _button.onClick.RemoveAllListeners();
    }
}

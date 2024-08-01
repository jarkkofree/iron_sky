using System;
using UnityEngine;
using UnityEngine.UI;

public class AddCreditsButton : MonoBehaviour
{
    private Button _button;

    public static Action OnClicked;

    private void Awake()
    {
        _button = GetComponent<Button>();
        _button.onClick.AddListener(() =>
        {
            _button.interactable = false;
            OnClicked?.Invoke();
        });
    }

    private void OnDestroy()
    {
        _button.onClick.RemoveAllListeners();
    }
}

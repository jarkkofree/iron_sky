using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

public class RefineryButton : MonoBehaviour
{
    [SerializeField] private Button _button;

    public static Action OnButtonClicked;

    private void Awake()
    {
        _button.onClick.AddListener(()=>
        {
            _button.interactable = false;
            OnButtonClicked?.Invoke();
        });
    }

    private void OnDestroy()
    {
        _button.onClick.RemoveAllListeners();
    }
}

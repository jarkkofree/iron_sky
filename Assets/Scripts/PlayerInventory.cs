using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

[Serializable]
public class PlayerInventory
{
    [SerializeField] private int _credits;

    public static Action<int> OnCreditsChanged;

    public void AddCredits(int credits)
    {
        if (credits > 0)
        {
            _credits += credits;
            OnCreditsChanged?.Invoke(_credits);
        }
    }
}

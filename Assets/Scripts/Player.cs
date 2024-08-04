using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Player : MonoBehaviour
{
    private static Player _player;
    private PlayerInventory _inventory = new PlayerInventory();

    public static PlayerInventory Inventory => GetInventory();

    private void Awake()
    {
        if (_player == null)
            _player = this;
    }

    private static PlayerInventory GetInventory()
    {
        if ( _player == null)
            return new PlayerInventory();

        return _player._inventory;
    }
}

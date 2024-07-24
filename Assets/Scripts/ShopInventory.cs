using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;
using Random = UnityEngine.Random;

[Serializable]
public class ShopInventory
{
    [SerializeField] private List<ShopItem> _items;

    public List<ShopItem> RandomizeInventory()
    {
        HashSet<ShopItem> items = new HashSet<ShopItem>();

        if (_items == null
            || _items.Count <= 0) return items.ToList();

        int totalItems = Random.Range(1, _items.Count);

        while(items.Count < totalItems)
        {
            int randomItem = Random.Range(0, _items.Count);
            ShopItem item = _items[randomItem];

            if (item != null)
                items.Add(item);
        }

        return items.ToList();
    }
}

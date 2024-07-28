using UnityEngine;
using UnityEngine.Serialization;

[CreateAssetMenu(fileName = "ShopData", menuName = "ShopUI/Data/Shop")]
public class ShopData : ScriptableObject
{
    [SerializeField] private string _displayName;
    [SerializeField] private ShopInventory _inventory;

    public string DisplayName => _displayName;
    public ShopInventory Inventory => _inventory;
}
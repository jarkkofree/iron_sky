using UnityEngine;
using UnityEngine.Serialization;

[CreateAssetMenu(fileName = "Shop", menuName = "ShopUI/Shop")]
public class Shop : ScriptableObject
{
    [SerializeField] private string _displayName;
    [SerializeField] private ShopInventory _inventory;

    public string DisplayName => _displayName;
    public ShopInventory Inventory => _inventory;
}
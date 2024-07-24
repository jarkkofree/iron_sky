using UnityEngine;
using UnityEngine.Serialization;

[CreateAssetMenu(fileName = "ShopItem", menuName = "ShopUI/ShopItem")]
public class ShopItem : ScriptableObject
{
    [SerializeField] private string _displayName;
    [SerializeField] private float _basePrice;

    public string DisplayName => _displayName;
    public float BasePrice => _basePrice;
}

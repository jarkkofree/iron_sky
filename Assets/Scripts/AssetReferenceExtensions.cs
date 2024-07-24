using System.Collections;
using System.Collections.Generic;
using System.Threading.Tasks;
using UnityEngine;
using UnityEngine.AddressableAssets;

public static class AssetReferenceExtensions
{
    public static async Task<T> InstantiateAsyncGetComponent<T>(this AssetReference reference) where T : Component
    {
        GameObject go = await reference.InstantiateAsync().Task;

        Component component = go.GetComponent<T>();
        return component as T;
    }

    public static async Task<T> InstantiateAsyncGetComponentInChildren<T>(this AssetReference reference) where T : Component
    {
        GameObject go = await reference.InstantiateAsync().Task;

        Component component = go.GetComponentInChildren<T>();
        return component as T;
    }
}

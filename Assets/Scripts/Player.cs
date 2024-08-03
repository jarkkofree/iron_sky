using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Player : MonoBehaviour
{
    [SerializeField] private float _credits;

    public float Credits => _credits;

    // Start is called before the first frame update
    void Start()
    {
        _credits = 0;
    }

    // Update is called once per frame
    void Update()
    {
        
    }

    public void AddCredits()
    {
        _credits += 1;
    }
}

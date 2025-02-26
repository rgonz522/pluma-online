import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';

@Component({
  selector: 'pluma-online-register',
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.scss']
})
export class RegisterComponent implements OnInit {
  public registerForm: FormGroup = new FormGroup({
    email: new FormControl(''),
    password: new FormControl('')
  });

  public isSubmitted: boolean = false;

  constructor() {}

  public ngOnInit(): void {}

  public onSubmit() {}
}

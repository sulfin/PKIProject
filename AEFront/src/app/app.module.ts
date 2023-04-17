import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { EmailFormulaireComponent } from './email-formulaire/email-formulaire.component';
import { AccueilComponent } from './accueil/accueil.component';
import {MatIconModule} from "@angular/material/icon";
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { RevokeFormulaireComponent } from './revoke-formulaire/revoke-formulaire.component';
import {MatInputModule} from "@angular/material/input";
import { ConfirmationMailComponent } from './confirmation-mail/confirmation-mail.component';
import {MatButtonModule} from "@angular/material/button";
import { FormDownloadComponent } from './form-download/form-download.component';
import {FormsModule} from "@angular/forms";
import {HttpClientModule} from "@angular/common/http";


@NgModule({
  declarations: [
    AppComponent,
    EmailFormulaireComponent,
    AccueilComponent,
    RevokeFormulaireComponent,
    ConfirmationMailComponent,
    FormDownloadComponent
  ],
    imports: [
        BrowserModule,
        AppRoutingModule,
        MatIconModule,
        BrowserAnimationsModule,
        MatInputModule,
        MatButtonModule,
        FormsModule,
        HttpClientModule

    ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }

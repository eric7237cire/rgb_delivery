import { TestBed } from '@angular/core/testing';

import { GridStorageService } from './grid-storage.service';

describe('GridStorageService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: GridStorageService = TestBed.get(GridStorageService);
    expect(service).toBeTruthy();
  });
});

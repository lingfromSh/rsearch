import * as React from 'react';
import Box from '@mui/material/Box';

import Container from '@mui/material/Container';
import Button from '@mui/material/Button';
import { Grid, TextField, List, ListItem, ListItemText, Pagination } from '@mui/material';

export default class App extends React.PureComponent {

  state = {
    pageSize: 10,
    pageIndex: 0,
    pageTotal: 0,
    searchResult: []
  }

  handleSearch = (e, value) => {
    console.log(e.key === 'Enter');
    if (e.key === 'Enter') {
      console.log(this.state.searchResult);
      let newSearchResult = [...this.state.searchResult];
      newSearchResult.push({
        id: '001',
        title: '如何学好Rust',
      });
      this.setState({
        searchResult: newSearchResult
      })
      this.setState({pageIndex: 1, pageTotal: Math.ceil(newSearchResult.length / this.state.pageSize)});
    }
  };

  handlePageChange = (e, value) => {
    console.log(e, value);
    this.setState({
      pageIndex: value
    })
  };

  render() {
    const {searchResult, pageTotal, pageIndex} = this.state;
    return (
      <Box
        marginTop={2}
      >
        <Container>
          <Grid item xl={8} xs={10} alignContent='center'>
            <TextField
              id='outlined-basic'
              label='type something here'
              fullWidth
              size='small'
              onKeyUp={this.handleSearch}
            />
          </Grid>
          {
            searchResult.length !== 0 ? (<Grid> <List dense>
              {searchResult.map((r) => {
                return (
                  <ListItem secondaryAction={<Button>VIEW</Button>}>
                    <ListItemText>{r.title}</ListItemText>
                  </ListItem>
                );
              })}
            </List>
            <Pagination page={pageIndex} count={pageTotal} onChange={this.handlePageChange} showFirstButton showLastButton /></Grid>) : <div></div>
          }

        </Container>
      </Box>
    );
  }
}
